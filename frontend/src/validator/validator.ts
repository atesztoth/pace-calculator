import type { Validator, ValidatableObject, ValidatedResult } from './types'
import { ValidatorKey } from './validatable-types.enum'
import { validateStringNumber, validateStringNumberGtZero, validateTimeString } from './validators'

const defaultValidators = Object.freeze({
  stringTime: validateTimeString,
  stringNumber: validateStringNumber,
  stringNumberGtZero: validateStringNumberGtZero,
} satisfies Record<keyof typeof ValidatorKey, Validator<any>>)

export const runValidate = <
  T extends ValidatableObject,
  K extends keyof T,
  V extends Record<keyof typeof ValidatorKey, Validator<any>>,
>(
  object: T,
  validators: V = defaultValidators as V, // added this here so it is actually testable, Jest was dying on this.
  // But I also want this to be hidden from the users, so nice, I can now curry this function for no particular reason other than Jest :)
  // Sometimes tests do nothing just holds you back.
): ValidatedResult<K> => {
  return (Object.keys(object) as K[]).reduce((a, c) => {
    // No constraints? Ok, valid.
    if (object[c].constraints.length < 1) {
      return { ...a, [c]: { isValid: true, validatorResults: [] } as ValidatedResult<string>[string] }
    }

    const validatorResults: { isValid: boolean; constraint: keyof typeof ValidatorKey }[] = object[c].constraints.map(
      (constraint) => ({ constraint, isValid: validators[constraint](object[c].value as any) }),
    )

    return { ...a, [c]: { isValid: validatorResults.reduce((a, c) => a && c.isValid, true), validatorResults } }
  }, {} as ValidatedResult<K>)
}

export const validate = <T extends ValidatableObject>(object: T) => runValidate(object, defaultValidators)
