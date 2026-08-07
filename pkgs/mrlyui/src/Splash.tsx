import { useCallback, useEffect, useRef, useState } from "react"
import type { ReactNode } from "react"
import { createPortal } from "react-dom"
import { cx, useMedia } from "./lib"

// SPLASH

export function Splash({ children, onDismiss, className }: {
  children?: ReactNode
  onDismiss?: () => void
  className?: string
}) {
  if (typeof document === "undefined") return null
  return createPortal(
    <div className={cx("splash", className)} onClick={onDismiss}>
      {children}
    </div>,
    document.body,
  )
}

// START

export function Start({ children = "start", onClick }: {
  children?: ReactNode
  onClick?: () => void
}) {
  if (typeof document === "undefined") return null
  return createPortal(
    <button type="button" className="button start" onClick={onClick}>
      {children}
    </button>,
    document.body,
  )
}

// ATTRACT

export function useAttract(armed: boolean, tick: () => void, pace = 250): { idle: boolean; wake: () => void } {
  const [idle, setIdle] = useState(false)
  const calm = useMedia("(prefers-reduced-motion: reduce)")
  const beat = useRef(tick)
  beat.current = tick

  useEffect(() => {
    setIdle(armed)
  }, [armed])

  useEffect(() => {
    if (!idle || calm) return
    const id = setInterval(() => beat.current(), pace)
    return () => clearInterval(id)
  }, [idle, calm, pace])

  const wake = useCallback(() => setIdle(false), [])
  return { idle, wake }
}
