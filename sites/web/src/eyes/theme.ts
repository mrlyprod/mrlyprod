import { setFills, sound } from "mrlyui"
import type { Fill } from "mrlyui"
import { refresh, setSeed, setSource } from "./wallpaper"

// NIGHT

export function dark(): boolean {
  if (typeof document === "undefined") return false
  const theme = document.documentElement.dataset.theme
  if (theme === "dark") return true
  if (theme === "light") return false
  return typeof matchMedia !== "undefined" && matchMedia("(prefers-color-scheme: dark)").matches
}

// LOOK

export function look(key: string, value: unknown): void {
  const root = document.documentElement
  switch (key) {
    case "darkmode":
      root.dataset.theme = value === true ? "dark" : "light"
      refresh()
      break
    case "color":
      root.style.setProperty("--accent-color", `var(--c-${String(value)})`)
      refresh()
      break
    case "scale":
      root.style.setProperty("--unit", `${Number(value)}px`)
      break
    case "radius":
      root.style.setProperty("--radius", `calc(var(--unit) * ${Number(value)})`)
      break
    case "pace":
      root.style.setProperty("--pace", `${Number(value)}ms`)
      break
    case "fill": {
      const fill = String(value) as Fill
      setFills([fill, fill, fill])
      break
    }
    case "background":
      root.style.setProperty("--background-color", `var(--c-${String(value)})`)
      refresh()
      break
    case "width":
      root.style.setProperty("--frame-width", `${Number(value)}px`)
      break
    case "font":
      root.style.setProperty("--font-body", `var(--font-${String(value)})`)
      break
    case "emoji":
      if (value === "system") root.style.removeProperty("--font-emoji")
      else root.style.setProperty("--font-emoji", `"${String(value)}"`)
      break
    case "wallpaper":
      setSource(String(value))
      break
    case "seed":
      setSeed(Number(value))
      break
    case "sound":
    case "haptics":
    case "note":
    case "wave":
    case "duration":
      sound.pref(key, value)
      break
  }
}
