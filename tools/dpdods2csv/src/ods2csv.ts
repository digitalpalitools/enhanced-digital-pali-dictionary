import * as fsApi from 'fs'
import util from 'util'
import logger from './Logger'
import * as Ods from './OdsProcessor'
import { dpsOds } from './OdsTypes/DpsOds'
import { dpdOds } from './OdsTypes/DpdOds'

const fs = {
  appendFile: util.promisify(fsApi.appendFile),
  copyFile: util.promisify(fsApi.copyFile),
  createWriteStream: fsApi.createWriteStream,
  mkdir: util.promisify(fsApi.mkdir),
  readFile: util.promisify(fsApi.readFile),
  writeFile: util.promisify(fsApi.writeFile),
}

export interface CommandArgs {
  odsFile: string
  sheetName: string
  columnCount: number
  odsType: string
}

export const runCommand = async (args: CommandArgs) => {
  logger.info('------------------------------')
  logger.info(
    // eslint-disable-next-line max-len
    `Executing with odsFile=${args.odsFile} sheetName=${args.sheetName} columnCount=${args.columnCount} odsType=${args.odsType}`,
  )
  logger.info('------------------------------')

  const reporter: Ods.Reporter = {
    Info: x => logger.info(x),
    Error: x => logger.error(x),
  }

  const odsType = args.odsType === 'dps' ? dpsOds : dpdOds
  const odsData = await fs.readFile(args.odsFile)
  const allWords = await Ods.readAllPaliWords(
    odsData,
    args.sheetName,
    args.columnCount,
    reporter,
    odsType.paliWordFactory,
  )

  allWords.forEach(w => {
    logger.info(`>>> ${w.sortKey()} => ${w.tocId()}`)
  })

  const fullCsv = Ods.generateFullCsv(allWords, reporter)
  fs.writeFile(args.odsFile.replace(/.ods$/i, '-full.csv'), fullCsv)

  const vocabCsv = Ods.generateVocabCsv(allWords, reporter)
  fs.writeFile(args.odsFile.replace(/.ods$/i, '-vocab.csv'), vocabCsv)

  const rootCsv = Ods.generateRootCsv(allWords, reporter)
  fs.writeFile(args.odsFile.replace(/.ods$/i, '-root.csv'), rootCsv)
}
