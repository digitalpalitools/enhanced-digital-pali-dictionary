export interface PaliWordBase {
  isValidWord(): boolean
  sortKey(): string
  groupId(): string
  tocId(): string
  includeInRootCsv(): boolean
  includeInDictionary(): boolean
  toCsvRow(): string
  createTocSummary(): string
  createWordData(): string
}

export type PaliWordFields = { [colName: string]: number }

export type PaliWordFactory = (h: PaliWordFields, r: string[]) => PaliWordBase

export const padTrailingNumbers = (s: string) => s.replace(/\d+/g, m => '00'.substr(m.length - 1) + m)

export const makeGroupId = (baseGroupId: string) => {
  const parts = baseGroupId.split(' ')

  if (!Number.isNaN(parseInt(parts[parts.length - 1], 10))) {
    return parts.slice(0, parts.length - 1).join(' ')
  }

  return baseGroupId
}

export const toCsvRow = (records: string[]) => records.map(x => `"${x}"`).join('\t')
