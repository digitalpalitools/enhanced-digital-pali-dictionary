import type { Config } from '@jest/types'

const initialOptions = async (): Promise<Config.InitialOptions> => ({
  preset: 'ts-jest',
  testEnvironment: 'node',
  verbose: true,
})

export default initialOptions
