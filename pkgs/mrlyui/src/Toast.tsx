import { useEffect } from "react"
import type { ReactNode } from "react"
import { Symbol } from "./Glyphs"
import { cx } from "./lib"
import { VARIANT_ICON, type Variant } from "./variant"

export function Toast({ open, children, variant, duration = 2500, onClose }: {
  open: boolean
  children: ReactNode
  variant?: Variant
  duration?: number
  onClose: () => void
}) {
  useEffect(() => {
    if (!open) return
    const id = setTimeout(onClose, duration)
    return () => clearTimeout(id)
  }, [open, children, duration, onClose])
  if (!open) return null
  return (
    <div className="toast-layer">
      <div className={cx("toast", variant)} role="status">
        {variant && (
          <span className="toast-icon" aria-hidden="true">
            <Symbol name={VARIANT_ICON[variant]} size="var(--font-size)" />
          </span>
        )}
        <span>{children}</span>
      </div>
    </div>
  )
}
