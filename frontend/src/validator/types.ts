import { ValidatorKey } from './validatable-types.enum'

export type ValidationResult<T = unknown> = {
  /**
   * @description Tells if the value is valid according to the validator.
   */
  isValid: boolean

  /**
   * @description If the validator decides, it can share the value it used to validate. (If valid)
   */
  value?: T | null
}

export type Validator<T extends unknown[], R = unknown> = (...input: T) => ValidationResult<R>

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

export type ValidatedResult<T extends string | number | symbol, R = unknown> = {
  [K in T]: {
    isValid: boolean
    validatorResults: {
      /**
       * @description Tells if the data is valid according to the validator.
       */
      isValid: boolean

      /**
       * @description Sets which validator to run.
       */
      constraint: keyof typeof ValidatorKey

      /**
       * @description The value the validator may share after validation. Requires the user to know the validator!
       */
      validatorValue: R | null
    }[]
  }
}
