/**
 * Tries to detect if the input string contains a valid integer of a float.
 * Tries to parse correctly.
 * @param str
 */
export const parseNumberStr = (str: string): number | null => {
  if (str.trim() === '') return null

  const seemsFloat = /[.,]/.test(str)
  const parsed = !seemsFloat ? parseInt(str, 10) : parseFloat(str)
  return !isNaN(parsed) && isFinite(parsed) ? parsed : null
}
