import { ValidatorKey } from './validatable-types.enum'

export type Validator<T extends unknown[]> = (...input: T) => boolean

export type ValidatableObject = {
  [K: string]: {
    value: unknown
    constraints: (keyof typeof ValidatorKey)[]
  }
}

export type ValidatedResult<T extends string | number | symbol> = {
  [K in T]: { isValid: boolean; validatorResults: { isValid: boolean; constraint: keyof typeof ValidatorKey }[] }
}
