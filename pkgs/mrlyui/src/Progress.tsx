import type { CSSProperties } from "react"
import { cx } from "./lib"

export function Progress({ value, max = 100 }: {
  value?: number
  max?: number
}) {
  const done = value === undefined ? undefined : Math.max(0, Math.min(100, (value / max) * 100))
  return (
    <div
      className={cx("progress", done === undefined && "indeterminate")}
      role="progressbar"
      aria-valuemin={0}
      aria-valuemax={max}
      aria-valuenow={done === undefined ? undefined : value}
    >
      <div className="progress-bar" style={done === undefined ? undefined : ({ "--progress": `${done}%` } as CSSProperties)} />
    </div>
  )
}
