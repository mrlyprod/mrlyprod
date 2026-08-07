import type { CSSProperties, ReactNode } from "react"
import { cx } from "./lib"

// BOARD

export function Board({ cols, rows, children, className }: {
  cols: number
  rows: number
  children?: ReactNode
  className?: string
}) {
  const shape = { "--board-cols": cols, "--board-rows": rows } as CSSProperties
  return (
    <div className={cx("board", className)} style={shape}>
      {children}
    </div>
  )
}

// CELL

export function Cell({ x, y, z, onClick, children }: {
  x: number
  y: number
  z?: number
  onClick?: () => void
  children?: ReactNode
}) {
  const spot: CSSProperties = { gridColumn: x + 1, gridRow: y + 1, zIndex: z }
  if (onClick) {
    return (
      <button type="button" className="cell" style={spot} onClick={onClick}>
        {children}
      </button>
    )
  }
  return (
    <div className="cell" style={spot}>
      {children}
    </div>
  )
}
