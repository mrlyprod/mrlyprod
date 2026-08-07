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

export function Cell({ x, y, z, on, bg, onClick, children, className }: {
  x?: number
  y?: number
  z?: number
  on?: boolean
  bg?: string
  onClick?: () => void
  children?: ReactNode
  className?: string
}) {
  const spot: CSSProperties = {}
  if (x !== undefined) spot.gridColumn = x + 1
  if (y !== undefined) spot.gridRow = y + 1
  if (z !== undefined) spot.zIndex = z
  if (bg !== undefined) spot.backgroundColor = bg
  const skin = cx("cell", on && "on", className)
  if (onClick) {
    return (
      <button type="button" className={skin} style={spot} onClick={onClick}>
        {children}
      </button>
    )
  }
  return (
    <div className={skin} style={spot}>
      {children}
    </div>
  )
}
