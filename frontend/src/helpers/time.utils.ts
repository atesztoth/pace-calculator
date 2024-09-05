import type { Seconds } from './types'

export const timeRegex = new RegExp(/^([0-9]*)(:[0-5][0-9])?(:[0-5][0-9])?$/)

/**
 * Tries to convert strings that match the `timeRegex` pattern to seconds
 * Returns null if parsing fails.
 *
 * Arbitrary number of hours are accepted and because the regexp is in the form that is has now,
 * it'll accept arbitrary seconds if only 1 params are passed. I think it's all right like this
 * (convenient) but may change this later by introducing whole patterns for cases,
 * like: [0-5][0-9] for seconds, (:[0-5][0-9])(:[0-5][0-9]) for minutes e.g...
 */
export const convertTimeStrToSeconds = (str: string): Seconds | null => {
  if (!str) return null
  const matches = timeRegex.exec(str)
  if (!matches || matches.length < 1) return null

  // Magic comes
  // Actually, not that big of a magic. Just extracting the matched values.
  // The only problem is they don't have a fixed place, the smaller value places will
  // shift to the left the more, the fewer matches there are. No worries, just
  // reverse them, and give the proper place values, done!
  const [, a, b, c] = matches

  return [a, b, c]
    .map((s) => (s ? parseInt(s.replace(/:/g, '')) : null))
    .filter((v) => v !== null && !isNaN(v) && isFinite(v))
    .reverse()
    .reduce((a, c, i) => a! + c! * Math.pow(60, i), 0)
}

export const secondsToTimeString = (input: number): string => {
  const { hours, minutes, seconds } = secondsToHMS(input)
  console.info({ hours, minutes, seconds })
  return [hours, minutes, seconds]
    .filter((v, i, a) => v !== 0 || i === a.length - 1)
    .map((v) => String(v).padStart(2, '0'))
    .join(':')
}

export const secondsToHMS = (input: number): { hours: number; minutes: number; seconds: number } => {
  const hours = Math.floor(input / 3600)
  const minutes = Math.floor((input % 3600) / 60)
  const seconds = input % 60

  return { hours, minutes, seconds }
}
