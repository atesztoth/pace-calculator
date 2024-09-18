import { parseNumberStr } from './number.utils'

describe('Number utils', () => {
  describe('parseNumberStr', () => {
    const cases: [input: string, result: number | null][] = [
      ['0', 0],
      ['-0', -0],
      ['0.12345', 0.12345],
      ['-0.12345', -0.12345],
      ['1', 1],
      ['10000000000', 10000000000],
    ]

    test.each(cases)('parses %s into %d', (input, result) => {
      expect(parseNumberStr(input)).toEqual(result)
    })
  })
})
