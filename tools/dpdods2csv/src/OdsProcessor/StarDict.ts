import * as luxon from 'luxon'
import { Buffer } from 'buffer/'
import { PaliWordBase } from './PaliWord'
import { Reporter } from './Common'
import { OdsType } from './OdsType'

type PaliWordGroup = { [id: string]: PaliWordBase[] }

interface IdxWord {
  str: string
  dataOffset: number
  dataSize: number
}

interface IdxInfo {
  wordCount: number
  fileSize: number
}

const createHtml = (contents: string, odsType: OdsType) => `
<!DOCTYPE html>
<html>
<head>
<style>
* {
  font-family: "Verajja Serif", "DejaVu Sans", sans-serif;
}
table.word-info-table-${odsType.shortName} tr {
  vertical-align: top;
}
table.word-info-table-${odsType.shortName} tr td:nth-child(1), span.sutta-source-${odsType.shortName} {
  color: ${odsType.accentColor};
}
</style>
</head>
<body>
${contents}
</body>
</html>`

const createHtmlForWordGroup = (ws: PaliWordBase[], odsType: OdsType): string => {
  const sws = ws.sort((w1, w2) => w1.sortKey().localeCompare(w2.sortKey()))

  const toc = ws.length < 2 ? '' : `${sws.map((w) => w.createTocSummary()).join('\n')}<br/>`

  const html = sws.map((w) => w.createWordData()).join('')

  return createHtml(`${toc}${html}`, odsType)
}

const createDict = (odsType: OdsType, wordGroups: PaliWordGroup, reporter: Reporter): [IdxWord[], Buffer] => {
  reporter.Info(`... Creating dict: ${Object.keys(wordGroups).length} word groups.`)

  type IdBufferMap = { [id: string]: Buffer }
  const buffers = Object.entries(wordGroups).reduce((acc, [id, ws]) => {
    const html = createHtmlForWordGroup(ws, odsType)
    acc[id] = Buffer.from(html, 'utf-8')
    return acc
  }, {} as IdBufferMap)

  const idxWords = new Array<IdxWord>(Object.keys(buffers).length)
  Object.entries(buffers).forEach(([pali1, b], i) => {
    idxWords[i] = {
      str: pali1,
      dataOffset: i === 0 ? 0 : idxWords[i - 1].dataOffset + idxWords[i - 1].dataSize,
      dataSize: b.length,
    }
  })

  return [idxWords, Buffer.concat(Object.entries(buffers).map(([, b]) => b))]
}

// From https://stackoverflow.com/a/13225961/6196679
// eslint-disable-next-line @typescript-eslint/naming-convention
const g_ascii_strcasecmp = (s1: string, s2: string): number => {
  const str1 = Buffer.from(s1, 'utf-8')
  const str2 = Buffer.from(s2, 'utf-8')

  const n1 = str1.length
  const n2 = str2.length
  const min = Math.min(n1, n2)
  for (let i = 0; i < min; i += 1) {
    const c1 = str1.readUInt8(i)
    const c2 = str2.readUInt8(i)
    if (c1 !== c2) {
      // If non-ASCII char
      if (c1 > 127 || c2 > 127) {
        return c1 - c2
      }

      const c1char = String.fromCharCode(c1)
      const c2char = String.fromCharCode(c2)
      if (c1char.toUpperCase() !== c2char.toUpperCase()) {
        if (c1char.toLowerCase() !== c2char.toLowerCase()) {
          return c1 - c2
        }
      }
    }
  }

  return n1 - n2
}

const createIdx = (idxWords: IdxWord[], reporter: Reporter): [IdxInfo, Buffer] => {
  reporter.Info(`... Creating idx: ${idxWords.length} words.`)

  const sortedIdxWords = idxWords.sort((w1, w2) => g_ascii_strcasecmp(w1.str, w2.str))

  const buffers = sortedIdxWords.flatMap((w) => {
    const strB = Buffer.from(w.str, 'utf-8')

    const metaB = Buffer.alloc(1 + 4 + 4, 0, 'binary')
    metaB.writeInt32BE(w.dataOffset, 1)
    metaB.writeInt32BE(w.dataSize, 1 + 4)

    return [strB, metaB]
  })

  const buffer = Buffer.concat(buffers)

  return [{ wordCount: idxWords.length, fileSize: buffer.length }, buffer]
}

const createIfo = (odsType: OdsType, timeStamp: luxon.DateTime, idxInfo: IdxInfo, reporter: Reporter): Buffer => {
  reporter.Info(`... Creating ifo.`)

  const ifoContents = `StarDict's dict ifo file
version=${process.env.npm_package_version}
bookname=${odsType.name}
wordcount=${idxInfo.wordCount}
idxfilesize=${idxInfo.fileSize}
author=${odsType.author}
website=https://github.com/digitalpalitools/dpt-tools
description=${odsType.description}
date=${timeStamp.toUTC().toISO({ suppressMilliseconds: true })}
sametypesequence=h
`
  return Buffer.from(ifoContents, 'utf-8')
}

const copyIcon = async (odsType: OdsType, reporter: Reporter): Promise<Buffer> => {
  reporter.Info(`... Creating icon.`)

  return Buffer.from(odsType.icon, 'base64')
}

const readWordGroups = (allWords: PaliWordBase[], reporter: Reporter): PaliWordGroup => {
  const words = allWords.reduce((acc, e) => {
    const gid = e.groupId()
    acc[gid] = acc[gid] || []
    acc[gid].push(e)
    return acc
  }, {} as PaliWordGroup)

  reporter.Info(`... Grouped ${allWords.length} words into ${Object.keys(words).length} groups.`)

  return words
}

export type DigitalPaliDictionary = { [k: string]: Buffer }

export const generate = async (
  odsType: OdsType,
  allWords: PaliWordBase[],
  timeStamp: luxon.DateTime,
  reporter: Reporter,
): Promise<DigitalPaliDictionary> => {
  const wordGroups = readWordGroups(allWords, reporter)
  reporter.Info(`... Creating dictionary at '${odsType.shortName}'.`)
  const [idxWords, dictFile] = createDict(odsType, wordGroups, reporter)
  const [idx, idxFile] = createIdx(idxWords, reporter)
  const ifoFile = createIfo(odsType, timeStamp, idx, reporter)
  const iconFile = await copyIcon(odsType, reporter)

  return { dict: dictFile, idx: idxFile, ifo: ifoFile, png: iconFile }
}
