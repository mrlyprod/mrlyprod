import { useCallback, useEffect, useState } from "react"
import { read, write } from "./lib"
import type { SymbolName } from "./Glyphs"

// THEME

export type Theme = "" | "light" | "dark"

export const THEME_ICONS: Record<Theme, SymbolName> = {
  "": "brightness_auto",
  light: "light_mode",
  dark: "dark_mode",
}

const THEME = "mrly-theme"

export function useTheme(): [Theme, () => void] {
  const [theme, set] = useState<Theme>(() => {
    const held = read(THEME)
    return held === "light" || held === "dark" ? held : ""
  })
  useEffect(() => {
    const root = document.documentElement
    if (theme === "") delete root.dataset["theme"]
    else root.dataset["theme"] = theme
    write(THEME, theme)
  }, [theme])
  const cycle = useCallback(() => {
    set(held => (held === "" ? "light" : held === "light" ? "dark" : ""))
  }, [])
  return [theme, cycle]
}

// FONT

const FONT = "mrly-font"

export function useFont(): [boolean, () => void] {
  const [on, set] = useState(() => read(FONT) === "1")
  useEffect(() => {
    document.documentElement.classList.toggle("mrlyfont", on)
    write(FONT, on ? "1" : "")
  }, [on])
  const toggle = useCallback(() => set(held => !held), [])
  return [on, toggle]
}

// SCROLL

export function useScrolled(): void {
  useEffect(() => {
    const root = document.documentElement
    const mark = (): void => {
      root.classList.toggle("scrolled", window.scrollY > 48)
    }
    addEventListener("scroll", mark, { passive: true })
    mark()
    return () => {
      removeEventListener("scroll", mark)
    }
  }, [])
}
