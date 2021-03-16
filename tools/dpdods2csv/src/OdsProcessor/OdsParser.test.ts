import fs from 'fs'
import util from 'util'
import { Reporter } from './Common'
import { createInMemoryCSV, getBoldStyles, getCellText, getRowsInSheet, parseContentsXML } from './OdsParser'

const sheetName = 'PALI-X'
const odsFile = './src/OdsProcessor/testdata/Pali_English_Dictionary_10_rows.ods'
const boldStylesInOds = [
  'ce10',
  'ce108',
  'ce113',
  'ce117',
  'ce21',
  'ce31',
  'ce4',
  'ce41',
  'ce45',
  'ce50',
  'ce55',
  'ce64',
  'ce67',
  'ce74',
  'ce87',
  'ce96',
  'ce99',
  'T3',
  'T5',
]

const reporter: Reporter = {
  Info: () => {},
  Error: (message: any) => console.error(message),
}

test('can obtain boldStyles', async () => {
  const odsData = await util.promisify(fs.readFile)(odsFile)
  const doc = await parseContentsXML(odsData, reporter)

  const boldStyles = doc ? getBoldStyles(doc) : []

  expect(boldStyles.sort((a, b) => a.localeCompare(b))).toEqual(boldStylesInOds)
})

test('can obtain rows', async () => {
  const odsData = await util.promisify(fs.readFile)(odsFile)
  const doc = await parseContentsXML(odsData, reporter)

  const rows = doc ? getRowsInSheet(doc, sheetName, reporter) : []

  expect(rows).toHaveLength(11)
})

test('getCellText - simple', async () => {
  const odsData = await util.promisify(fs.readFile)(odsFile)
  const doc = await parseContentsXML(odsData, reporter)
  const rows = (doc ? getRowsInSheet(doc, sheetName, reporter) : []) || []

  const cell = rows[5].getElementsByTagName('table:table-cell')[4]

  const text = getCellText(cell, boldStylesInOds)

  expect(text).toEqual('adj, pp of bahulīkaroti, comp vb')
})

test('getCellText - with bold and breaks', async () => {
  const odsData = await util.promisify(fs.readFile)(odsFile)
  const doc = await parseContentsXML(odsData, reporter)
  const rows = (doc ? getRowsInSheet(doc, sheetName, reporter) : []) || []

  const text = createInMemoryCSV(rows, boldStylesInOds, 40)[6][30]

  expect(text).toEqual(
    // eslint-disable-next-line max-len
    `jiṇṇo'ham'asmi <b>abalo</b> vītavaṇṇo,<br/>nettā na suddhā savanaṃ na phāsu,<br/>m'āhaṃ nassaṃ momuho antarāva,<br/>ācikkha dhammaṃ yam'ahaṃ vijaññaṃ,<br/>jātijarāya idha vippahānaṃ.`,
  )
})
