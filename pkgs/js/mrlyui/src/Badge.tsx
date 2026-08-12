import type { ReactNode } from "react"
import { cx } from "./lib"
import type { Variant } from "./variant"

export function Badge({ children, variant, dot = false }: {
  children?: ReactNode
  variant?: Variant
  dot?: boolean
}) {
  return <span className={cx("badge", variant, dot && "dot")}>{!dot && children}</span>
}
