import { useId, useRef } from "react"
import type { ReactNode } from "react"
import { createPortal } from "react-dom"
import { Symbol } from "./Glyphs"
import { cx } from "./lib"
import { useOverlay } from "./Modal"

export function Drawer({
  open,
  onClose,
  side = "right",
  title,
  children,
}: {
  open: boolean
  onClose: () => void
  side?: "left" | "right"
  title?: string
  children: ReactNode
}) {
  const panel = useRef<HTMLDivElement>(null)
  const id = useId()
  useOverlay(open, onClose, panel)
  if (!open || typeof document === "undefined") return null
  return createPortal(
    <div
      className={cx("scrim", side)}
      onMouseDown={event => {
        if (event.target === event.currentTarget) onClose()
      }}
    >
      <div
        ref={panel}
        className={cx("drawer", side)}
        role="dialog"
        aria-modal="true"
        aria-labelledby={title === undefined ? undefined : id}
        tabIndex={-1}
      >
        <div className="overlay-head">
          {title !== undefined && (
            <span className="overlay-title" id={id}>
              {title}
            </span>
          )}
          <button type="button" className="overlay-close" onClick={onClose} aria-label="Close">
            <Symbol name="close" />
          </button>
        </div>
        {children}
      </div>
    </div>,
    document.body,
  )
}
