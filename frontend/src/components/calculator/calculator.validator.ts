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

  // Create the response by collecting the first (and now only lol) validators results
  // under the appropriate key:
  return {
    success: providedValueNames.reduce((a, key) => {
      const validatorResult = validationResult[key as keyof typeof validationResult]

      // Any error I throw here should never happen.
      // (Heuristics. If happens, well that's not good for many reasons.)
      if (!validatorResult || !validatorResult.isValid) throw new Error('Unexpected error!')

      const firstValidatorResult = validatorResult.validatorResults[0]?.validatorValue
      if (!firstValidatorResult) throw new Error('Unexpected error!')

      return { ...a, [key]: key === 'distance' ? +firstValidatorResult * 1000 : firstValidatorResult }
    }, {} as CalculatorInput),
  }
}
