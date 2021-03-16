import yargs from 'yargs'
import * as GF from './ods2csv'

const { argv } = yargs(process.argv.slice(2))
  .command({
    command: 'ods2csv [ods-file] [sheet-name] [column-count] [ods-type]',
    describe: 'Generate DPD CSV, vocab and root CSVs from DPD ODS.',
    builder: ya =>
      ya
        .default('ods-file', '/mnt/d/delme/dicts/Pali_English_Dictionary_10_rows.ods')
        .default('sheet-name', 'PALI-X')
        .default('column-count', 40)
        .default('ods-type', 'dpd'),
    handler: args =>
      GF.runCommand({
        odsFile: args['ods-file'],
        sheetName: args['sheet-name'],
        columnCount: args['column-count'],
        odsType: args['ods-type'],
      } as GF.CommandArgs),
  })
  .demandCommand(1)
  .strict()
  .help()
  .wrap(120)

if (!argv._[0]) {
  yargs.showHelp()
}
