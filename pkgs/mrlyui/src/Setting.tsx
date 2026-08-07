import type { ReactNode } from "react"
import { cx } from "./lib"

export function Setting({ label, hint, children, className }: {
  label: string
  hint?: string
  children?: ReactNode
  className?: string
}) {
  return (
    <div className={cx("setting", className)}>
      <div className="setting-lead">
        <span className="setting-label">{label}</span>
        {hint && <span className="setting-hint">{hint}</span>}
      </div>
      <div className="setting-control">{children}</div>
    </div>
  )
}
