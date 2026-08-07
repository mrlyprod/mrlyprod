import type { ReactNode } from "react"
import { cx } from "./lib"
import { Symbol } from "./Glyphs"
import type { SymbolName } from "./Glyphs"

export function Fold({ icon, label, open = false, grid = false, children }: {
  icon: SymbolName
  label: string
  open?: boolean
  grid?: boolean
  children?: ReactNode
}) {
  return (
    <details className="fold" open={open}>
      <summary>
        <Symbol name={icon} />
        {label}
        <Symbol name="chevron_right" className="turn" />
      </summary>
      <div className={cx("fold-body", grid && "fold-grid")}>{children}</div>
    </details>
  )
}
