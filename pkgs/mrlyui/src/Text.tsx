import type { CSSProperties, ReactNode } from "react"
import { cx } from "./lib"

// TEXT

export function Title({ children, className, style }: {
  children?: ReactNode
  className?: string
  style?: CSSProperties
}) {
  return (
    <div className={cx("title", className)} style={style}>
      {children}
    </div>
  )
}

export function Text({ children, className, style }: {
  children?: ReactNode
  className?: string
  style?: CSSProperties
}) {
  return (
    <div className={cx("text", className)} style={style}>
      {children}
    </div>
  )
}

export function Caption({ children, className, style }: {
  children?: ReactNode
  className?: string
  style?: CSSProperties
}) {
  return (
    <div className={cx("caption", className)} style={style}>
      {children}
    </div>
  )
}
