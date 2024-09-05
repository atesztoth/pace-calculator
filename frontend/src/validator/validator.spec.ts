import type { ValidatableObject } from './types'
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
      stringPace: jest.fn().mockReturnValue(true),
      stringTime: jest.fn().mockReturnValue(false),
      stringNumberGtZero: jest.fn().mockReturnValue(true),
      stringNumber: jest.fn().mockReturnValue(true),
    })

    expect(result).toEqual({
      a: {
        isValid: true,
        validatorResults: [
          {
            constraint: 'stringNumber',
            isValid: true,
          },
          {
            constraint: 'stringNumberGtZero',
            isValid: true,
          },
        ],
      },
      b: {
        isValid: true,
        validatorResults: [
          {
            constraint: 'stringNumberGtZero',
            isValid: true,
          },
        ],
      },
      c: {
        isValid: false,
        validatorResults: [
          {
            constraint: 'stringNumberGtZero',
            isValid: true,
          },
          {
            constraint: 'stringTime',
            isValid: false,
          },
        ],
      },
    })
  })
})
