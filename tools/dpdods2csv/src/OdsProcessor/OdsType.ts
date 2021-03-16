import { PaliWordFactory } from './PaliWord'

export interface OdsType {
  name: string
  shortName: string
  author: string
  description: string
  icon: string
  accentColor: string
  paliWordFactory: PaliWordFactory
}
