/* eslint-disable import/no-extraneous-dependencies */
import * as path from 'path'
import * as webpack from 'webpack'
import TsconfigPathsPlugin from 'tsconfig-paths-webpack-plugin'

delete process.env.TS_NODE_PROJECT

export const config: webpack.Configuration = {
  entry: './src/index.ts',
  mode: 'production',
  target: 'node',
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: [
          {
            loader: 'ts-loader',
            options: {
              configFile: path.resolve(__dirname, './tsconfig.json'),
            },
          },
        ],
      },
    ],
  },
  output: {
    path: path.resolve(__dirname, 'build'),
    filename: 'index.js',
  },
  resolve: {
    extensions: ['.ts', '.js'],
    mainFields: ['main'],
    plugins: [
      new TsconfigPathsPlugin({
        baseUrl: path.resolve(__dirname, '.'),
        configFile: path.resolve(__dirname, './tsconfig.json'),
      }),
    ],
  },
}

export default config
