import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "react"
import type { ReactNode } from "react"
import { createPortal } from "react-dom"
import { cx } from "./lib"

const GAP = 6

const EDGE = 8

export function Popover({
  trigger,
  children,
  align = "left",
  up = false,
}: {
  trigger: (args: { open: boolean; toggle: () => void }) => ReactNode
  children: ReactNode | ((close: () => void) => ReactNode)
  align?: "left" | "right"
  up?: boolean
}) {
  const [open, setOpen] = useState(false)
  const anchor = useRef<HTMLDivElement>(null)
  const panel = useRef<HTMLDivElement>(null)
  const close = useCallback(() => setOpen(false), [])
  const toggle = useCallback(() => setOpen(held => !held), [])

  const place = useCallback(() => {
    const box = anchor.current
    const pane = panel.current
    if (box === null || pane === null) return
    const rect = box.getBoundingClientRect()
    pane.style.setProperty("--pop-min", `${rect.width}px`)
    const width = pane.offsetWidth
    const height = pane.offsetHeight
    const x = align === "right" ? rect.right - width : rect.left
    const y = up ? rect.top - height - GAP : rect.bottom + GAP
    pane.style.setProperty("--pop-x", `${Math.max(EDGE, Math.min(x, window.innerWidth - width - EDGE))}px`)
    pane.style.setProperty("--pop-y", `${Math.max(EDGE, Math.min(y, window.innerHeight - height - EDGE))}px`)
  }, [align, up])

  useLayoutEffect(() => {
    if (open) place()
  })

  useEffect(() => {
    if (!open) return
    const down = (event: PointerEvent): void => {
      const node = event.target
      if (!(node instanceof Node)) return
      if (anchor.current?.contains(node) === true || panel.current?.contains(node) === true) return
      close()
    }
    const key = (event: KeyboardEvent): void => {
      if (event.key === "Escape") close()
    }
    document.addEventListener("pointerdown", down)
    document.addEventListener("keydown", key)
    window.addEventListener("scroll", place, true)
    window.addEventListener("resize", place)
    return () => {
      document.removeEventListener("pointerdown", down)
      document.removeEventListener("keydown", key)
      window.removeEventListener("scroll", place, true)
      window.removeEventListener("resize", place)
    }
  }, [open, close, place])

  return (
    <div className="popover-anchor" ref={anchor}>
      {trigger({ open, toggle })}
      {open &&
        createPortal(
          <div ref={panel} className={cx("popover", up && "up")}>
            {typeof children === "function" ? children(close) : children}
          </div>,
          document.body,
        )}
    </div>
  )
}
