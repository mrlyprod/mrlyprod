import { useEffect } from "react"
import { cx } from "./lib"
import { Letters } from "./Letters"

export type HeaderPane = "" | "menu" | "iden"

export function Header({ open = "", onMenu, onMark, onIden, className }: {
  open?: HeaderPane
  onMenu?: () => void
  onMark?: () => void
  onIden?: () => void
  className?: string
}) {
  useEffect(() => {
    document.documentElement.style.setProperty("--head-offset", "var(--head)")
    return () => {
      document.documentElement.style.removeProperty("--head-offset")
    }
  }, [])
  return (
    <header className={cx("header", className)}>
      <button
        type="button"
        className="header-glyph"
        aria-label={open === "menu" ? "Close menu" : "Open menu"}
        aria-expanded={open === "menu"}
        onClick={onMenu}
      >
        <Letters text={open === "menu" ? "×" : "+"} pace={150} label="" />
      </button>
      <button type="button" className="header-glyph header-mark" aria-label="mrly" onClick={onMark}>
        <Letters text="X" scramble={false} label="" />
      </button>
      <button
        type="button"
        className="header-glyph"
        aria-label={open === "iden" ? "Close identity" : "Open identity"}
        aria-expanded={open === "iden"}
        onClick={onIden}
      >
        <Letters text={open === "iden" ? "×" : "O"} pace={150} label="" />
      </button>
    </header>
  )
}
