import { useEffect } from "react"
import { cx } from "./lib"
import { Letters } from "./Letters"

export function Header({ menu = false, iden = false, onMenu, onMark, onIden, className }: {
  menu?: boolean
  iden?: boolean
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
        aria-label={menu ? "Close menu" : "Open menu"}
        aria-expanded={menu}
        onClick={onMenu}
      >
        <Letters text={menu ? "×" : "+"} pace={150} label="" />
      </button>
      <button type="button" className="header-glyph header-mark" aria-label="mrly" onClick={onMark}>
        <Letters text="X" scramble={false} label="" />
      </button>
      <button
        type="button"
        className="header-glyph"
        aria-label={iden ? "Close identity" : "Open identity"}
        aria-expanded={iden}
        onClick={onIden}
      >
        <Letters text={iden ? "×" : "O"} pace={150} label="" />
      </button>
    </header>
  )
}
