import type { ValidationResult } from './types'
import { validateStringNumber, validateStringNumberGtZero } from './validators'

describe('Validators', () => {
  describe('validateStringNumber', () => {
    const cases: [input: string, result: ValidationResult<number>][] = [
      ['', { isValid: false, value: null }],
      ['0', { isValid: true, value: 0 }],
      ['-0', { isValid: true, value: -0 }], // this is acceptable
      ['1', { isValid: true, value: 1 }],
    ]

    test.each(cases)('should convert %s to %o', (input, result) => {
      expect(validateStringNumber(input)).toEqual(result)
    })
  })

  describe('validateStringNumberGtZero', () => {
    const cases: [input: string, result: ValidationResult<number>][] = [
      ['', { isValid: false, value: null }],
      ['0', { isValid: false, value: null }],
      ['-0', { isValid: false, value: null }],
      ['1', { isValid: true, value: 1 }],
      ['-1', { isValid: false, value: null }],
    ]

    test.each(cases)('should convert %s to %o', (input, result) => {
      expect(validateStringNumberGtZero(input)).toEqual(result)
    })
  })
})
