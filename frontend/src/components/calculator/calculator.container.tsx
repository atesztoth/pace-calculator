import type { DisplayableCalculatorResult, CalculatorInput } from './types'
import { createPromiseSleep } from '../../util/utils'
import { secondsToTimeString } from '../../helpers/time.utils'
import { CalculatorResult, useSendCalculation } from '../../hooks/send-calculation.hook'

import toast from 'react-hot-toast'
import React, { useState } from 'react'
import CalculatorComponent from './calculator.component'

const CalculatorContainer: React.FC = () => {
  const [results, setResults] = useState<DisplayableCalculatorResult | undefined>(void 'result')
  const [isLoading, setIsLoading] = useState(false)

  const calculate = useSendCalculation()

  const onCalculate = (input: CalculatorInput) => {
    setIsLoading(true)

    // To avoid jumping loading state:
    Promise.all([createPromiseSleep(150), calculate(input)])
      .then(([, results]) => {
        setResults(convertResults(results))
      })
      .catch((e) => {
        toast.error(e.message)
      })
      .finally(() => {
        setIsLoading(false)
      })
  }

  return (
    <CalculatorComponent
      parentLoading={isLoading}
      onCalculate={onCalculate}
      onClearResults={() => {
        setResults(void 0)
      }}
      presentingResults={results}
    />
  )
}

export default CalculatorContainer

const convertResults = (results: CalculatorResult): DisplayableCalculatorResult => {
  return {
    distance: results.distance / 1000,
    time: secondsToTimeString(results.time),
    pace: secondsToTimeString(results.pace),
  }
}
