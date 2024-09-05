import type { ValidationResult } from './types'
import { parseNumberStr } from '../helpers/number.utils'
import { convertTimeStrToSeconds, timeRegex } from '../helpers/time.utils'

// In some validators possibly you have to actually parse the value to something
// like in validateStringPace, so as a possible improvement it would be nice to
// expose the results of validators either to each other (and create a sequential validator
// that not necessarily, but can work sequentially on previous validators results), or expose
// them to the final result. Both are possible and would work well.

// But... I just accidentally developed this validator, so ... these features may not be implemented
// at all. But ... hmm maybe it wouldn't be that bad of an idea to create a simple lib from it :thinking:

// Update: added the feature of exposing transformed values from validators so users
// potentially can skip converting the values themselves at the space of usage

export const validateStringNumber = (v: string): ValidationResult<number> => {
  const parsedNumber = parseNumberStr(v)
  const isValid = parsedNumber !== null

  return { isValid, value: isValid ? parsedNumber : null }
}

export const validateStringNumberGtZero = (v: string): ValidationResult<number> => {
  const parsedNumber = parseNumberStr(v)
  const isValid = parsedNumber !== null && parsedNumber > 0

  return { isValid, value: isValid ? parsedNumber : null }
}

export const validateTimeString = (str: string): ValidationResult<null> => {
  return { isValid: !!str && (timeRegex.exec(str)?.length ?? 0) > 0, value: null }
}

export const validateStringPace = (str: string): ValidationResult<number> => {
  const convertedValue = !!str ? (convertTimeStrToSeconds(str) ?? 0) : 0
  const isValid = convertedValue > 0

  return { isValid, value: convertedValue }
}
