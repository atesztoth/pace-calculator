import type { Validator, ValidatableObject, ValidatedResult } from './types'
import { ValidatorKey } from './validatable-types.enum'
import { validateStringPace, validateTimeString, validateStringNumber, validateStringNumberGtZero } from './validators'

const defaultValidators = Object.freeze({
  stringTime: validateTimeString,
  stringPace: validateStringPace,
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
  // Sometimes tests do nothing just hold you back.
): ValidatedResult<K> => {
  return (Object.keys(object) as K[]).reduce((a, c) => {
    if ((object[c].excludeIfUndefined ?? false) && object[c].value === void 0) return a

    // No constraints? Ok, valid.
    if (object[c].constraints.length < 1) {
      return { ...a, [c]: { isValid: true, validatorResults: [] } as ValidatedResult<string>[string] }
    }

    const validatorResults: ValidatedResult<string>[string]['validatorResults'][0][] = object[c].constraints.map(
      (constraint) => {
        const validationResult = validators[constraint](object[c].value as any)
        return { constraint, isValid: validationResult.isValid, validatorValue: validationResult.value }
      },
    )

    return { ...a, [c]: { isValid: validatorResults.reduce((a, c) => a && c.isValid, true), validatorResults } }
  }, {} as ValidatedResult<K>)
}

export const validate = <K extends string = string>(object: ValidatableObject<K>) =>
  runValidate(object, defaultValidators)
