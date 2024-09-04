import { Toaster } from 'react-hot-toast'

import React from 'react'
import logo from './running-run.svg'
import CalculatorContainer from './components/calculator/calculator.container'

import './App.css'

function App() {
  return (
    <div className="App">
      <main className="App-main">
        <div className="App-content">
          <img src={logo} className="App-logo" alt="logo" style={{ color: 'white', fill: 'white' }} />
          <h1>Pace calculator</h1>
          <CalculatorContainer />
        </div>
      </main>
      <Toaster />
    </div>
  )
}

export default App
