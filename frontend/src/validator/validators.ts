import { parseNumberStr } from '../helpers/number.utils'
import { convertTimeStrToSeconds, timeRegex } from '../helpers/time.utils'

// In some validators possibly you have to actually parse the value to something
// like in validateStringPace, so as a possible improvement it would be nice to
// expose the results of validators either to each other (and create a sequential validator
// that not necessarily, but can work sequentially on previous validators results), or expose
// them to the final result. Both are possible and would work well.

// But... I just accidentally developed this validator, so ... these features may not be implemented
// at all. But ... hmm maybe it wouldn't be that bad of an idea to create a simple lib from it :thinking:

export const validateStringNumber = (v: string): boolean => {
  return parseNumberStr(v) !== null
}

export const validateStringNumberGtZero = (v: string): boolean => {
  const parsedNumber = parseNumberStr(v)
  return parsedNumber !== null && parsedNumber > 0
}

export const validateTimeString = (str: string): boolean => {
  return !!str && (timeRegex.exec(str)?.length ?? 0) > 0
}

export const validateStringPace = (str: string): boolean => {
  return !!str && (convertTimeStrToSeconds(str) ?? 0) > 0
}
