type Keys = 'distance' | 'time' | 'pace'

// Pace must be in seconds!
export type CalculatorInput = { [K in Keys]?: number }
export type RawCalculatorInput = { [K in Keys]?: string | null }

export type DisplayableCalculatorResult = {
  distance: number // IN KM! The user has to see it in KM!
  time: string // In a properly formatted time string! e.g.: 1:00:11
  pace: string // Also a time string
}
