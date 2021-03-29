import fs from 'fs'
import util from 'util'
import { Reporter } from './Common'
import * as Ods from './OdsParser'

const sheetName = 'PALI-X'
const odsFile = './src/OdsProcessor/testdata/Pali_English_Dictionary_10_rows.ods'
const boldStylesInOds = [
  'ce103',
  'ce115',
  'ce119',
  'ce141',
  'ce144',
  'ce149',
  'ce22',
  'ce27',
  'ce38',
  'ce49',
  'ce59',
  'ce62',
  'ce68',
  'ce72',
  'ce83',
  'ce86',
  'ce93',
  'T3',
  'T5',
]

const reporter: Reporter = {
  Info: () => {},
  Error: (message: any) => console.error(message),
}

test('can obtain boldStyles', async () => {
  const odsData = await util.promisify(fs.readFile)(odsFile)
  const doc = await Ods.parseContentsXML(odsData, reporter)

  const boldStyles = doc ? Ods.getBoldStyles(doc) : []

  expect(boldStyles.sort((a, b) => a.localeCompare(b))).toEqual(boldStylesInOds)
})

test('can obtain rows', async () => {
  const odsData = await util.promisify(fs.readFile)(odsFile)
  const doc = await Ods.parseContentsXML(odsData, reporter)

  const rows = doc ? Ods.getRowsInSheet(doc, sheetName, reporter) : []

  expect(rows).toHaveLength(11)
})

const getRowsInSheet = async (): Promise<Element[]> => {
  const odsData = await util.promisify(fs.readFile)(odsFile)
  const doc = await Ods.parseContentsXML(odsData, reporter)
  const rows = (doc ? Ods.getRowsInSheet(doc, sheetName, reporter) : []) || []
  return rows
}

const createInMemoryCSV = async (): Promise<string[][]> => {
  const rows = await getRowsInSheet()

  const text = Ods.createInMemoryCSV(rows, boldStylesInOds, 40)
  return text
}

test('getCellText - simple', async () => {
  const rows = await getRowsInSheet()

  const cell = rows[5].getElementsByTagName('table:table-cell')[4]

  const text = Ods.getCellText(cell, boldStylesInOds)

  expect(text).toEqual('adj, pp of bahulīkaroti, comp vb')
})

test('getCellText - with bold and breaks', async () => {
  const text = (await createInMemoryCSV())[6][30]

  expect(text).toEqual(
    // eslint-disable-next-line max-len
    `jiṇṇo'ham'asmi <b>abalo</b> vītavaṇṇo,<br/>nettā na suddhā savanaṃ na phāsu,<br/>m'āhaṃ nassaṃ momuho antarāva,<br/>ācikkha dhammaṃ yam'ahaṃ vijaññaṃ,<br/>jātijarāya idha vippahānaṃ.`,
  )
})

test('getCellText - with double quotes', async () => {
  const text = (await createInMemoryCSV())[5][30]

  expect(text).toEqual(
    // eslint-disable-next-line max-len
    `evam'eva kho, bhikkhave, ""yassa"" 'kassaci bhikkhuno 'mettācetovimutti' abhāvitā <b>abahulīkatā</b> so suppadhaṃsiyo hoti ""amanussehi.`,
  )
})
