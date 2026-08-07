import { useCallback, useEffect, useState, useSyncExternalStore } from "react"
import { read, write } from "./lib"
import * as sound from "./sound"
import type { ColorName } from "./colors"

export type Fill = ColorName | "random" | ""

export type Prefs = {
  unit: number
  border: number
  radius: number
  accent: ColorName | ""
  fill: Fill
  background: ColorName | ""
  sound: boolean
  haptics: boolean
  note: string
  wave: string
  duration: number
}

export const PREF_DEFAULTS: Prefs = {
  unit: 0,
  border: 1,
  radius: 2,
  accent: "",
  fill: "",
  background: "",
  sound: true,
  haptics: true,
  note: "random",
  wave: "sine",
  duration: 150,
}

let fillHeld: Fill = ""
const fillSubs = new Set<() => void>()

export function setFill(next: Fill): void {
  if (next === fillHeld) return
  fillHeld = next
  for (const sub of fillSubs) sub()
}

export function useFill(): Fill {
  return useSyncExternalStore(
    sub => {
      fillSubs.add(sub)
      return () => fillSubs.delete(sub)
    },
    () => fillHeld,
  )
}

const KEY = "mrly-prefs"

function set(name: string, value: string) {
  if (value === "") document.documentElement.style.removeProperty(name)
  else document.documentElement.style.setProperty(name, value)
}

export function loadPrefs(): Prefs {
  const held = read(KEY)
  if (held === "") return PREF_DEFAULTS
  try {
    return { ...PREF_DEFAULTS, ...(JSON.parse(held) as Partial<Prefs>) }
  } catch {
    return PREF_DEFAULTS
  }
}

export function applyPrefs(prefs: Prefs): void {
  set("--unit", prefs.unit === 0 ? "" : `${prefs.unit}px`)
  set("--border-width", prefs.border === 1 ? "" : `${prefs.border}px`)
  set("--radius", prefs.radius === 2 ? "" : `calc(var(--unit) * ${prefs.radius})`)
  set("--accent-color", prefs.accent === "" ? "" : `var(--c-${prefs.accent})`)
  set("--background-color", prefs.background === "" ? "" : `var(--c-${prefs.background})`)
  setFill(prefs.fill)
  sound.pref("sound", prefs.sound)
  sound.pref("haptics", prefs.haptics)
  sound.pref("note", prefs.note)
  sound.pref("wave", prefs.wave)
  sound.pref("duration", prefs.duration)
}

export function usePrefs(): [Prefs, (patch: Partial<Prefs>) => void, () => void] {
  const [prefs, setPrefs] = useState(loadPrefs)

  useEffect(() => {
    applyPrefs(prefs)
  }, [prefs])

  const patch = useCallback((part: Partial<Prefs>) => {
    setPrefs(held => {
      const next = { ...held, ...part }
      write(KEY, JSON.stringify(next))
      return next
    })
  }, [])

  const reset = useCallback(() => {
    write(KEY, "")
    setPrefs(PREF_DEFAULTS)
  }, [])

  return [prefs, patch, reset]
}
