import { useEffect, useId, useRef } from "react"
import type { ReactNode, RefObject } from "react"
import { createPortal } from "react-dom"
import { Symbol } from "./Glyphs"

const FOCUS =
  'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])'

let locks = 0

function grab(panel: RefObject<HTMLDivElement | null>): HTMLElement[] {
  const root = panel.current
  return root === null ? [] : Array.from(root.querySelectorAll<HTMLElement>(FOCUS))
}

export function useOverlay(open: boolean, onClose: () => void, panel: RefObject<HTMLDivElement | null>): void {
  useEffect(() => {
    if (!open) return
    const before = document.activeElement instanceof HTMLElement ? document.activeElement : null
    locks += 1
    document.body.classList.add("no-scroll")
    const first = grab(panel)[0] ?? panel.current
    first?.focus()
    const key = (event: KeyboardEvent): void => {
      if (event.key === "Escape") {
        event.preventDefault()
        onClose()
        return
      }
      if (event.key !== "Tab") return
      const items = grab(panel)
      const head = items[0]
      const tail = items[items.length - 1]
      if (head === undefined || tail === undefined) {
        event.preventDefault()
        panel.current?.focus()
        return
      }
      const active = document.activeElement
      const outside = !(active instanceof Node) || panel.current?.contains(active) !== true
      if (event.shiftKey && (outside || active === head)) {
        event.preventDefault()
        tail.focus()
      } else if (!event.shiftKey && (outside || active === tail)) {
        event.preventDefault()
        head.focus()
      }
    }
    document.addEventListener("keydown", key)
    return () => {
      document.removeEventListener("keydown", key)
      locks -= 1
      if (locks === 0) document.body.classList.remove("no-scroll")
      before?.focus()
    }
  }, [open, onClose, panel])
}

export function Modal({
  open,
  onClose,
  title,
  children,
}: {
  open: boolean
  onClose: () => void
  title?: string
  children: ReactNode
}) {
  const panel = useRef<HTMLDivElement>(null)
  const id = useId()
  useOverlay(open, onClose, panel)
  if (!open || typeof document === "undefined") return null
  return createPortal(
    <div
      className="scrim"
      onMouseDown={event => {
        if (event.target === event.currentTarget) onClose()
      }}
    >
      <div
        ref={panel}
        className="modal"
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
