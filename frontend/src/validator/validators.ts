import { timeRegex } from '../helpers/time.utils'
import { parseNumberStr } from '../helpers/number.utils'

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
