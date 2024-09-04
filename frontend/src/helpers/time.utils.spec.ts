import { convertTimeStrToSeconds } from './time.utils'

describe('Time utils', () => {
  describe('convertTimeStrToSeconds', () => {
    const cases: [str: string, result: null | number][] = [
      ['', null],
      ['invalid', null],
      ['0', 0],
      ['100', 100],
      ['1000', 1000],
      ['1:1000', null],
      ['01:1000', null],
      ['1:10', 70],
      ['01:10', 70],
      ['1:01:10', 3670],
    ]

    test.each(cases)('expect %s to be converted to %d', (input, output) => {
      expect(output).toEqual(convertTimeStrToSeconds(input))
    })
  })
})
