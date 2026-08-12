import type { ReactNode } from "react"
import { Symbol } from "./Glyphs"
import { cx } from "./lib"
import { VARIANT_ICON, type Variant } from "./variant"

export function Alert({ children, title, variant = "info", onClose }: {
  children: ReactNode
  title?: string
  variant?: Variant
  onClose?: () => void
}) {
  return (
    <div className={cx("alert", variant)} role="alert">
      <span className="alert-icon" aria-hidden="true">
        <Symbol name={VARIANT_ICON[variant]} />
      </span>
      <div className="alert-body">
        {title && <span className="alert-title">{title}</span>}
        <span className="alert-text">{children}</span>
      </div>
      {onClose && (
        <button type="button" className="dismiss" onClick={onClose} aria-label="Dismiss">
          <Symbol name="close" size="var(--font-size)" />
        </button>
      )}
    </div>
  )
}
