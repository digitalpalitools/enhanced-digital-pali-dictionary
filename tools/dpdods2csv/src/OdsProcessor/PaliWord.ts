export interface PaliWordBase {
  sortKey(): string
  groupId(): string
  tocId(): string
  includeInRootCsv(): boolean
  includeInDictionary(): boolean
  toCsvRow(): string
  createTocSummary(): string
  createWordData(): string
}

export type PaliWordFactory = (x: string[]) => PaliWordBase

export const padTrailingNumbers = (s: string) => s.replace(/\d+/g, (m) => '00'.substr(m.length - 1) + m)

export const makeGroupId = (baseGroupId: string) => {
  const parts = baseGroupId.split(' ')

  if (!Number.isNaN(parseInt(parts[parts.length - 1], 10))) {
    return parts.slice(0, parts.length - 1).join(' ')
  }

  return baseGroupId
}

export const toCsvRow = (records: string[]) => records.map((x) => `"${x}"`).join('\t')
