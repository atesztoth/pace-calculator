import type { CalculatorInput, RawCalculatorInput } from './types'

export const enum CalculationValidationError {
  invalidInput = 'InvalidInput',
  invalidPaceValue = 'invalidPaceValue',
  mismatchedInputCount = 'MismatchedInputCount',
}

export const validateCalculatorInputs = (
  input: RawCalculatorInput,
): { success?: CalculatorInput; error?: CalculationValidationError } => {
  throw new Error('IMPLEMENT ME!')
  // const providedValueNames = Object.entries(input)
  //   .filter(([, v]) => !!v)
  //   .map(([k]) => k)
  //
  // if (providedValueNames.length !== 2) {
  //   return { error: CalculationValidationError.mismatchedInputCount }
  // }
  //
  // // const numberValuedKeys = Object.keys(input).filter((k) => k !== PACE_KEY)
  // const { result, error } = providedValueNames.reduce(
  //   (a, key) => {
  //     if (!!a.error) return a
  //
  //     if (key === PACE_KEY) {
  //       const paceSeconds = convertTimeStrToSeconds(input[key] || '') // used || instead of ?? purposefully
  //       return paceSeconds === null
  //         ? { error: CalculationValidationError.invalidPaceValue }
  //         : { ...a, result: { ...a.result, [PACE_KEY]: paceSeconds } }
  //     }
  //
  //     const rawValue = input[key as keyof typeof input]
  //     if (!rawValue) return a
  //
  //     const { value, isValid } = validateStringNumber(input[key as keyof typeof input]!, (n) => n > 0)
  //
  //     return isValid
  //       ? { ...a, result: { ...a.result, [key]: value! } }
  //       : { error: CalculationValidationError.invalidInput }
  //   },
  //   { result: void 0, error: void 0 } as {
  //     result?: CalculatorInput
  //     error?: CalculationValidationError
  //   },
  // )
  //
  // if (error) return { error }
  //
  // return { success: result! }
}
