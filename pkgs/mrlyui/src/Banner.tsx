import type { ReactNode } from "react"
import { Symbol, type SymbolName } from "./Glyphs"
import { cx } from "./lib"
import type { Variant } from "./variant"

export function Banner({ children, variant, icon, onClose }: {
  children: ReactNode
  variant?: Variant
  icon?: SymbolName
  onClose?: () => void
}) {
  return (
    <div className={cx("banner", variant)} role="status">
      {icon ? (
        <span className="banner-icon" aria-hidden="true">
          <Symbol name={icon} />
        </span>
      ) : (
        <span className="banner-accent" aria-hidden="true" />
      )}
      <span className="banner-text">{children}</span>
      {onClose && (
        <button type="button" className="dismiss" onClick={onClose} aria-label="Dismiss">
          <Symbol name="close" size="var(--font-size)" />
        </button>
      )}
    </div>
  )
}
