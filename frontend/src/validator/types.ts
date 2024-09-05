import { ValidatorKey } from './validatable-types.enum'

export type Validator<T extends unknown[]> = (...input: T) => boolean

export type ValidatableObject<K extends string = string> = {
  [Key in K]: {
    /**
     * @description The value to validate
     */
    value: unknown

    /**
     * @description Names of the validation constraints
     */
    constraints: (keyof typeof ValidatorKey)[]

    /**
     * @description Tells if the value should be discarded from the result if undefined. False by default.
     */
    excludeIfUndefined?: boolean
  }
}

export type ValidatedResult<T extends string | number | symbol> = {
  [K in T]: { isValid: boolean; validatorResults: { isValid: boolean; constraint: keyof typeof ValidatorKey }[] }
}
