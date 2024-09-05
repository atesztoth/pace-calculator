import type { ValidatableObject } from '../../validator/types'
import type { CalculatorInput, RawCalculatorInput } from './types'
import { validate } from '../../validator/validator'

export const enum CalculationValidationError {
  invalidInput = 'InvalidInput',
  invalidPaceValue = 'invalidPaceValue',
  mismatchedInputCount = 'MismatchedInputCount',
}

export const validateCalculatorInputs = (
  input: RawCalculatorInput,
): { success?: CalculatorInput; error?: CalculationValidationError } => {
  // First let's check how many values did we even get:
  const providedValueNames = Object.entries(input)
    .filter(([, v]) => !!v)
    .map(([k]) => k)

  if (providedValueNames.length !== 2) {
    return { error: CalculationValidationError.mismatchedInputCount }
  }

  // Now we can handle further validation
  const validatableObject: ValidatableObject<keyof RawCalculatorInput> = {
    pace: { value: input.pace || void 0, excludeIfUndefined: true, constraints: ['stringPace'] },
    time: { value: input.time || void 0, excludeIfUndefined: true, constraints: ['stringNumberGtZero'] },
    distance: { value: input.distance || void 0, excludeIfUndefined: true, constraints: ['stringNumberGtZero'] },
  }

  const validationResult = validate(validatableObject)

  if (providedValueNames.includes('pace') && !validationResult.pace?.isValid) {
    return { error: CalculationValidationError.invalidPaceValue }
  }

  if (Object.values(validationResult).some((v) => !v.isValid)) {
    return { error: CalculationValidationError.invalidInput }
  }

  return {
    success: providedValueNames.reduce(
      (a, key) => ({ ...a, [key]: input[key as keyof typeof input] }),
      {} as CalculatorInput,
    ),
  }
}
