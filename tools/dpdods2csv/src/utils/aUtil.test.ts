import * as utils from './aUtil'

describe('callA()', () => {
  it('should return true', () => {
    expect(utils.callA()).toEqual(true)
  })
})

describe('callB()', () => {
  it('should return true', () => {
    expect(utils.callB()).toEqual(false)
  })
})
