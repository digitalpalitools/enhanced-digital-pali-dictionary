export interface Reporter {
  Info: (message: string) => void
  Error: (message: string) => void
}
