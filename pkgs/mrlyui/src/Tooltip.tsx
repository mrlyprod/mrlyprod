import type { ReactNode } from "react"
import { cx } from "./lib"

export function Tooltip({
  label,
  side = "top",
  children,
}: {
  label: ReactNode
  side?: "top" | "bottom" | "left" | "right"
  children: ReactNode
}) {
  return (
    <span className="tooltip" tabIndex={0}>
      {children}
      <span className={cx("tooltip-bubble", side !== "top" && side)} role="tooltip">
        {label}
      </span>
    </span>
  )
}
