import type { ReactNode } from "react"
import { cx } from "./lib"

const SINCE = 2013

export function Footer({ children, className }: {
  children?: ReactNode
  className?: string
}) {
  return (
    <footer className={cx("footer", className)}>
      {children}
      <p className="fine">
        Copyright © MrlyProd, Inc. {SINCE}-{new Date().getFullYear()}
      </p>
    </footer>
  )
}
