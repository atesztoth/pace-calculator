import { ValidatableObject, ValidationResult } from './types'
import { runValidate } from './validator'

describe('Validator', () => {
  test('validator', () => {
    const obj = {
      a: {
        value: '1',
        constraints: ['stringNumber', 'stringNumberGtZero'],
      },
      b: {
        value: '3',
        constraints: ['stringNumberGtZero'],
      },
      c: {
        value: '123:3',
        constraints: ['stringNumberGtZero', 'stringTime'],
      },
      d: {
        value: void 0,
        excludeIfUndefined: true,
        constraints: ['stringNumberGtZero'],
      },
    } satisfies ValidatableObject

    const result = runValidate(obj, {
      stringPace: jest.fn((x) => ({ isValid: true, value: x }) satisfies ValidationResult<any>),
      stringTime: jest.fn((x) => ({ isValid: false, value: null }) satisfies ValidationResult<any>),
      stringNumberGtZero: jest.fn((x) => ({ isValid: true, value: x }) satisfies ValidationResult<any>),
      stringNumber: jest.fn((x) => ({ isValid: true, value: x }) satisfies ValidationResult<any>),
    })

    expect(result).toEqual({
      a: {
        isValid: true,
        validatorResults: [
          {
            constraint: 'stringNumber',
            isValid: true,
            validatorValue: '1',
          },
          {
            constraint: 'stringNumberGtZero',
            isValid: true,
            validatorValue: '1',
          },
        ],
      },
      b: {
        isValid: true,
        validatorResults: [
          {
            constraint: 'stringNumberGtZero',
            isValid: true,
            validatorValue: '3',
          },
        ],
      },
      c: {
        isValid: false,
        validatorResults: [
          {
            constraint: 'stringNumberGtZero',
            isValid: true,
            validatorValue: '123:3',
          },
          {
            constraint: 'stringTime',
            isValid: false,
            validatorValue: null,
          },
        ],
      },
    })
  })
})
