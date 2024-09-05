export const parseNumberStr = (str: string): number | null => {
  const parsed = parseInt(str, 10)
  return !isNaN(parsed) && isFinite(parsed) ? parsed : null
}
