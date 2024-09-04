import type { CalculatorInput } from './types'
import { createPromiseSleep } from '../../util/utils'
import { useSendCalculation } from '../../hooks/send-calculation.hook'

import React, { useState } from 'react'
import CalculatorComponent from './calculator.component'
import toast from 'react-hot-toast'

const CalculatorContainer: React.FC = () => {
  const [isLoading, setIsLoading] = useState(false)

  const calculate = useSendCalculation()

  const onCalculate = (input: CalculatorInput) => {
    // TODO:
    setIsLoading(true)

    // To avoid jumping loading state:
    Promise.all([createPromiseSleep(150), calculate(input)])
      .then((results) => {
        alert(JSON.stringify(results))
      })
      .catch((e) => {
        toast.error(e.message)
      })
      .finally(() => {
        setIsLoading(false)
      })
  }

  return <CalculatorComponent parentLoading={isLoading} onCalculate={onCalculate} />
}

export default CalculatorContainer
