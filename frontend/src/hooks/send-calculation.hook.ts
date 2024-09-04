import type { Meter, Seconds } from '../helpers/types'
import type { CalculatorInput } from '../components/calculator/types'

export type CalculatorResult = {
  distance: Meter
  time: Seconds
  pace: Seconds
}

const url = `${process.env.REACT_APP_BACKEND_URL!}/calculate`
const apiKey = process.env.REACT_APP_API_KEY!

export const useSendCalculation = () => {
  return async (input: CalculatorInput): Promise<CalculatorResult> => {
    const result = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-API-KEY': apiKey,
      },
      body: JSON.stringify(input),
    })

    if (!result.ok) {
      console.error('Error:', result.status, result.statusText)
      throw new Error('Server error.')
    }

    return result.json()
  }
}
