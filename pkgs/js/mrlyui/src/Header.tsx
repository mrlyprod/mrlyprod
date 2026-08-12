import { useEffect, useRef } from "react"
import { cx } from "./lib"
import { Letters } from "./Letters"
import { usePlate, plated } from "./Box"
import { Grip } from "./Grip"
import type { PaneSet } from "./pane"

export function Header({ menu = false, iden = false, onMenu, onMark, onIden, panes, className }: {
  menu?: boolean
  iden?: boolean
  onMenu?: () => void
  onMark?: () => void
  onIden?: () => void
  panes?: PaneSet
  className?: string
}) {
  const bar = useRef<HTMLElement>(null)
  const color = usePlate(undefined, 2)

  useEffect(() => {
    const node = bar.current
    if (node === null) return
    const paint = () => {
      const rect = node.getBoundingClientRect()
      document.documentElement.style.setProperty("--head-offset", `${String(Math.round(rect.bottom))}px`)
    }
    const eye = new ResizeObserver(paint)
    eye.observe(node)
    paint()
    return () => {
      eye.disconnect()
      document.documentElement.style.removeProperty("--head-offset")
    }
  }, [])

  const docked = panes !== undefined && panes.wide
  const leftOpen = docked && panes.left.open
  const rightOpen = docked && panes.right.open

  return (
    <header className={cx("header", className)} style={plated(undefined, color)} ref={bar}>
      <button
        type="button"
        className={cx("header-glyph", leftOpen && "sized-left")}
        aria-label={menu ? "Close menu" : "Open menu"}
        aria-expanded={menu}
        onClick={onMenu}
      >
        <Letters text={menu ? "×" : "+"} pace={150} label="" />
      </button>
      {leftOpen && <Grip side="left" pane={panes.left} />}
      <button type="button" className="header-glyph header-mark" aria-label="mrly" onClick={onMark}>
        <Letters text="X" scramble={false} label="" />
      </button>
      {rightOpen && <Grip side="right" pane={panes.right} />}
      <button
        type="button"
        className={cx("header-glyph", rightOpen && "sized-right")}
        aria-label={iden ? "Close identity" : "Open identity"}
        aria-expanded={iden}
        onClick={onIden}
      >
        <Letters text={iden ? "×" : "O"} pace={150} label="" />
      </button>
    </header>
  )
}
