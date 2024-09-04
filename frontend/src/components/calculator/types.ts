type Keys = 'distance' | 'time' | 'pace'

// Pace must be in seconds!
export type CalculatorInput = { [K in Keys]?: number }
export type RawCalculatorInput = { [K in Keys]?: string | null }
