import JSZip from 'jszip'
import { DOMParser } from 'xmldom'
import { PaliWordBase, PaliWordFactory } from './PaliWord'
import { Reporter } from './Common'

type OdsParserInput = Blob | Uint8Array

export const parseContentsXML = async (file: OdsParserInput, reporter: Reporter): Promise<Document | null> => {
  const zip = await JSZip.loadAsync(file)

  const xmlStr = await zip.file('content.xml')?.async('string')
  if (!xmlStr) {
    reporter.Error('content.xml not found. Invalid ODS file.')
    return null
  }
  const parser = new DOMParser()
  const contentsXml = parser.parseFromString(xmlStr || 'not found - add error handling here', 'application/xml')

  return contentsXml
}

export const getBoldStyles = (contentsXml: Document): string[] => {
  const boldStyles = [] as string[]

  const allAutomaticStyles = contentsXml.getElementsByTagName('office:automatic-styles')
  Array.from(allAutomaticStyles)
    .map((e) => e as Element)
    .forEach((automaticStyles) => {
      Array.from(automaticStyles?.childNodes)
        .map((e) => e as Element)
        .forEach((automaticStyle) => {
          Array.from(automaticStyle?.childNodes)
            .map((e) => e as Element)
            .forEach((style) => {
              if (style?.tagName === 'style:text-properties' && style?.getAttribute('fo:font-weight') === 'bold') {
                const s = automaticStyle.getAttribute('style:name')
                if (s) {
                  boldStyles.push(s)
                }
              }
            })
        })
    })

  return boldStyles
}

export const getRowsInSheet = (contentsXml: Document, sheetName: string, reporter: Reporter): Element[] | null => {
  const allSpreadsheets = contentsXml.getElementsByTagName('office:spreadsheet')

  const firstSpreadsheet = allSpreadsheets[0]
  const table = Array.from(firstSpreadsheet.getElementsByTagName('table:table')).find(
    (t) => t.getAttribute('table:name') === sheetName,
  )

  if (!table) {
    reporter.Error(`Could not find sheet named ${sheetName}`)
    return null
  }

  const [, ...rows] = Array.from(table.getElementsByTagName('table:table-row'))

  return rows
}

const processText = (nodes: NodeListOf<ChildNode>, boldStyles: string[]): string => {
  const nodeTexts = Array.from(nodes).map((n) => {
    if (n.nodeType === n.TEXT_NODE) {
      return n.nodeValue
    }

    const ne = n as Element
    const isBold =
      ne && ne.tagName === 'text:span' && boldStyles.indexOf(ne.getAttribute('text:style-name') || '') !== -1
    const tag = isBold ? ['<b>', '</b>'] : ['', '']
    return `${tag[0]}${processText(ne.childNodes, boldStyles)}${tag[1]}`
  })

  return nodeTexts.join('')
}

export const getCellText = (cell: Element, boldStyles: string[]): string => {
  const cellContents = cell.getElementsByTagName('text:p')
  if (cellContents.length > 0) {
    return Array.from(cellContents)
      .map((x) => processText(x.childNodes, boldStyles))
      .join('<br/>')
  }

  return ''
}

export const createInMemoryCSV = (rows: Element[], boldStyles: string[], columnCount: number): string[][] => {
  const rowReducer = (rowAcc: string[][], row: Element) => {
    const baseCells = row.getElementsByTagName('table:table-cell')
    const flatCells = Array.from(baseCells).reduce((accFlatCell, cell) => {
      const repeatCount = parseInt(cell.getAttribute('table:number-columns-repeated') || '', 10) || 0
      if (repeatCount > 0) {
        const repCells = Array(repeatCount).fill(cell)
        return accFlatCell.concat(repCells)
      }

      return [...accFlatCell, cell]
    }, [] as Element[])

    const cells = flatCells.splice(0, columnCount).map((c) => getCellText(c, boldStyles))

    return [...rowAcc, cells]
  }

  return rows.reduce(rowReducer, [] as string[][])
}

export const readAllPaliWords = async (
  file: OdsParserInput,
  sheetName: string,
  columnCount: number,
  reporter: Reporter,
  pwFactory: PaliWordFactory,
): Promise<PaliWordBase[]> => {
  reporter.Info(`OdsProcessor: processODS: Starting processing.`)
  let start = Date.now()
  const contentsXml = await parseContentsXML(file, reporter)
  let end = Date.now()
  reporter.Info(`OdsProcessor: processODS: Parsed ODS. (${(end - start) / 1000.0} s)`)

  if (!contentsXml) {
    reporter.Error('Unable to parse content.xml')
    return Promise.reject()
  }

  start = Date.now()
  const boldStyles = getBoldStyles(contentsXml)
  end = Date.now()
  reporter.Info(`OdsProcessor: processODS: Obtained all bold styles. (${(end - start) / 1000.0} s)`)

  start = Date.now()
  const rows = getRowsInSheet(contentsXml, sheetName, reporter)
  end = Date.now()
  reporter.Info(`OdsProcessor: processODS: Obtaining sheet '${sheetName}'. (${(end - start) / 1000.0} s)`)

  if (!rows) {
    reporter.Error(`Unable to find sheet ${sheetName}`)
    return Promise.reject()
  }

  start = Date.now()
  const inMemCsv = createInMemoryCSV(rows, boldStyles, columnCount)
  end = Date.now()
  reporter.Info(
    `OdsProcessor: processODS: Created in memory csv with ${inMemCsv.length} rows. (${(end - start) / 1000.0} s)`,
  )

  return inMemCsv.map(pwFactory).filter((w) => w.includeInDictionary())
}
