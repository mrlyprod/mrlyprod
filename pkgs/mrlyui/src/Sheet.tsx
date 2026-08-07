import { useRef } from "react"
import type { ReactNode } from "react"
import { createPortal } from "react-dom"
import { useOverlay } from "./Modal"

export function Sheet({
  open,
  onClose,
  children,
}: {
  open: boolean
  onClose: () => void
  children: ReactNode
}) {
  const panel = useRef<HTMLDivElement>(null)
  useOverlay(open, onClose, panel)
  if (!open || typeof document === "undefined") return null
  return createPortal(
    <div
      className="scrim low"
      onMouseDown={event => {
        if (event.target === event.currentTarget) onClose()
      }}
    >
      <div ref={panel} className="sheet" role="dialog" aria-modal="true" tabIndex={-1}>
        {children}
      </div>
    </div>,
    document.body,
  )
}
