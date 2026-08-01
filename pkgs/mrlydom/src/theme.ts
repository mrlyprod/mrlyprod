import { useCallback, useEffect, useState } from "react"

// STORE

export const ICONS: Record<string, string> = {
  "": "brightness_auto",
  light: "light_mode",
  dark: "dark_mode",
}

const THEME = "mrly-theme"

const FONT = "mrly-font"

export function read(key: string): string {
  try {
    return localStorage.getItem(key) ?? ""
  } catch {
    return ""
  }
}

export function write(key: string, value: string): void {
  try {
    if (value === "") localStorage.removeItem(key)
    else localStorage.setItem(key, value)
  } catch {}
}

// THEME

export function useTheme(): [string, () => void] {
  const [theme, set] = useState(() => {
    const held = read(THEME)
    return held === "light" || held === "dark" ? held : ""
  })
  const cycle = useCallback(() => {
    set(held => {
      const next = held === "" ? "light" : held === "light" ? "dark" : ""
      const root = document.documentElement
      if (next === "") delete root.dataset["theme"]
      else root.dataset["theme"] = next
      write(THEME, next)
      return next
    })
  }, [])
  return [theme, cycle]
}

export function useFont(): () => void {
  return useCallback(() => {
    write(FONT, document.documentElement.classList.toggle("mrlyfont") ? "1" : "")
  }, [])
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
