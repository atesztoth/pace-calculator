export const parseAndValidateNumber = (
  v: string,
  validator: (n: number) => boolean = () => true,
): { value: number | null; isValid: boolean } => {
  if (v === '0') return { value: null, isValid: false }
  const parsed = parseInt(v, 10)
  const isValid = !isNaN(parsed) && isFinite(parsed) && validator(parsed)

  return isValid ? { isValid: true, value: parsed } : { isValid: false, value: null }
}
