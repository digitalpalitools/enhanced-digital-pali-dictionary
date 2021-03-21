import { Buffer } from 'buffer'
import { Reporter } from './Common'
import { PaliWordBase } from './PaliWord'

//
// NOTE: There are bunch of issues with this implementation, fix as required.
// 1. This will break if there is a quote inside any word.
// 2. Not a streaming API, duplicates the entire dataset in memory.
//

const generateCsv = (allWords: PaliWordBase[], filter: (w: PaliWordBase) => boolean): [Buffer, number] => {
  const csvRows = allWords.filter(filter).map(w => w.toCsvRow())

  const csvRowsStr = csvRows.join('\r\n')

  return [Buffer.from(csvRowsStr, 'utf8'), csvRows.length]
}

export const generateFullCsv = (allWords: PaliWordBase[], reporter: Reporter): Buffer => {
  reporter.Info('Creating full CSV')
  const [buffer, count] = generateCsv(allWords, () => true)
  reporter.Info(`... done. (${count} records)`)

  return buffer
}

export const generateVocabCsv = (allWords: PaliWordBase[], reporter: Reporter): Buffer => {
  reporter.Info('Creating vocab CSV')
  const [buffer, count] = generateCsv(allWords, w => w.includeInDictionary())
  reporter.Info(`... done. (${count} records)`)

  return buffer
}

export const generateRootCsv = (allWords: PaliWordBase[], reporter: Reporter): Buffer => {
  reporter.Info('Creating root CSV')
  const [buffer, count] = generateCsv(allWords, w => w.includeInDictionary() && !!w.includeInRootCsv())
  reporter.Info(`... done. (${count} records)`)

  return buffer
}
