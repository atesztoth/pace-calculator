import React, { useEffect } from 'react'

type Props = React.PropsWithChildren<{ onBackdropClick?: () => void; stopClickPropagation?: boolean }>
const SimpleModalComponent: React.FC<Props> = ({ children, onBackdropClick, stopClickPropagation = true }) => {
  useEffect(() => {
    document.body.style.overflow = 'hidden'

    return () => {
      document.body.style.overflow = 'unset'
    }
  })

  return (
    <div
      onClick={onBackdropClick}
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100dvw',
        height: '100dvh',
        background: 'rgba(0, 0, 0, 0.8)',
        overflow: 'hidden',
      }}
    >
      <div
        onClick={(event) => {
          if (stopClickPropagation) event.stopPropagation()
        }}
        style={{
          position: 'absolute',
          top: '50%',
          left: '50%',
          transform: 'translate(-50%, -50%)',
        }}
      >
        {children}
      </div>
    </div>
  )
}

export default SimpleModalComponent
