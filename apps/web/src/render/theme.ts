import { pref } from "../sound.ts"
import type { Held } from "../types.ts"
import { entries } from "./boards.ts"
import { pin } from "./paint.ts"
import { refresh, setSeed, setSource } from "./wallpaper.ts"

export function theme(key: string, value: unknown): void {
  const root = document.documentElement
  switch (key) {
    case "darkmode":
      document.body.classList.toggle("darkmode", value === true)
      refresh()
      break
    case "color":
      root.style.setProperty("--accent-color", `var(--c-${String(value)})`)
      for (const [surface] of entries()) (surface as unknown as Held).__committed = undefined
      refresh()
      break
    case "scale":
      root.style.setProperty("--unit-max", `${Number(value)}px`)
      break
    case "radius":
      root.style.setProperty("--border-radius", `calc(var(--unit) * ${Number(value)})`)
      break
    case "pace":
      root.style.setProperty("--pace", `${Number(value)}ms`)
      break
    case "fill":
      pin(value === "random" ? null : String(value))
      break
    case "background":
      document.body.style.setProperty("--background-color", `var(--c-${String(value)})`)
      refresh()
      break
    case "material":
      root.dataset.material = String(value)
      break
    case "wallpaper":
      setSource(String(value))
      break
    case "seed":
      setSeed(Number(value))
      break
    case "width":
      root.style.setProperty("--max-width", `${Number(value)}px`)
      break
    case "font":
      root.style.setProperty("--font", `var(--font-${String(value)})`)
      break
    case "emoji":
      if (value === "system") root.style.removeProperty("--font-emoji")
      else root.style.setProperty("--font-emoji", `"${String(value)}"`)
      break
    case "sound":
    case "haptics":
    case "note":
    case "wave":
    case "duration":
      pref(key, value)
      break
  }
}
