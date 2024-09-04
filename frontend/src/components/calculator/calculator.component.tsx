import type { CalculatorInput } from './types'
import { CalculationValidationError, validateCalculatorInputs } from './calculator.validator'

import toast from 'react-hot-toast'
import React, { useRef } from 'react'
import styles from './calculator.module.css'

type Props = { parentLoading: boolean; onCalculate: (_: CalculatorInput) => void }
const CalculatorComponent: React.FC<Props> = ({ parentLoading, onCalculate }) => {
  const distanceRef = useRef<HTMLInputElement>(null)
  const timeRef = useRef<HTMLInputElement>(null)
  const paceRef = useRef<HTMLInputElement>(null)

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    if (!distanceRef.current || !timeRef.current || !paceRef.current) {
      toast.error('Error! Please reload the page!')
      return
    }

    // Just a simple logic
    e.preventDefault()

    distanceRef.current.classList.remove('error')
    timeRef.current.classList.remove('error')
    paceRef.current.classList.remove('error')

    const { success: validValues, error } = validateCalculatorInputs({
      distance: distanceRef.current?.value,
      time: timeRef.current?.value,
      pace: paceRef.current?.value,
    })

    if (error) {
      distanceRef.current.classList.add('error')
      timeRef.current.classList.add('error')
      paceRef.current.classList.add('error')

      switch (error) {
        case CalculationValidationError.mismatchedInputCount: {
          toast.error('Please provide two values and leave one input empty!')
          return
        }
        case CalculationValidationError.invalidInput: {
          toast.error('Please make sure the provided values are greater than 0!')
          return
        }
        default: {
          toast.error('Unexpected error!')
          return
        }
      }
    }

    if (!!validValues!.pace) {
      // need to do a conversion too
      // const paceInSeconds
    }

    onCalculate(validValues!)
  }

  return (
    <div>
      <h2 className={styles.calculatorInfo}>Leave one of the fields empty to do the calculation!</h2>
      <form
        className={styles.formRoot}
        onSubmit={onSubmit}
        style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}
      >
        <div>
          <div className={styles.dataRoot}>
            <div className={styles.dataBox}>
              <span>I ran</span>
              <input disabled={parentLoading} ref={distanceRef} type="number" placeholder="10" />
            </div>
            <div className={styles.dataBox}>
              <span>kilometers in</span>
              <input disabled={parentLoading} ref={timeRef} type="number" placeholder="44" />
            </div>
            <div className={styles.dataBox}>
              <span>minutes, which is a pace of: </span>
              <input disabled={parentLoading} ref={paceRef} type="string" placeholder="1:04:24" />
              <span>(hour:min:sec) / km</span>
            </div>
          </div>
          <p className={styles.paceInfo}>In the pace column, hour and min are optional.</p>
        </div>
        <input disabled={parentLoading} type="submit" className={styles.calculatorSubmit} value="Calculate!" />
      </form>
    </div>
  )
}

export default CalculatorComponent
