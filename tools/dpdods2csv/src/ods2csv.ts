import logger from './Logger'

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
}
