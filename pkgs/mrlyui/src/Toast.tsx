import { useEffect, useRef } from "react"
import type { ReactNode } from "react"
import { createPortal } from "react-dom"
import { Symbol } from "./Glyphs"
import { cx } from "./lib"
import { VARIANT_ICON, type Variant } from "./variant"

let held: HTMLElement | null = null

function layer(): HTMLElement {
  if (held === null) {
    held = document.createElement("div")
    held.className = "toast-layer"
    document.body.append(held)
  }
  return held
}

export function Toast({ open, children, variant, duration = 2500, onClose }: {
  open: boolean
  children: ReactNode
  variant?: Variant
  duration?: number
  onClose: () => void
}) {
  const close = useRef(onClose)
  useEffect(() => {
    close.current = onClose
  }, [onClose])
  useEffect(() => {
    if (!open) return
    const id = setTimeout(() => close.current(), duration)
    return () => clearTimeout(id)
  }, [open, duration])
  if (!open || typeof document === "undefined") return null
  return createPortal(
    <div className={cx("toast", variant)} role="status">
      {variant && (
        <span className="toast-icon" aria-hidden="true">
          <Symbol name={VARIANT_ICON[variant]} size="var(--font-size)" />
        </span>
      )}
      <span>{children}</span>
    </div>,
    layer(),
  )
}
