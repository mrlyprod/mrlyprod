import { useCallback, useEffect, useSyncExternalStore } from "react"
import { read, write } from "./lib"
import * as sound from "./sound"
import type { ColorName } from "./colors"

export type Fill = ColorName | "random" | ""

export type Fills = [Fill, Fill, Fill]

export type Prefs = {
  unit: number
  border: number
  radius: number
  accent: ColorName | ""
  line: ColorName | ""
  fills: Fills
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
  line: "",
  fills: ["", "", ""],
  background: "",
  sound: true,
  haptics: true,
  note: "random",
  wave: "sine",
  duration: 150,
}

let fillsHeld: Fills = ["", "", ""]
const fillSubs = new Set<() => void>()

export function setFills(next: Fills): void {
  if (next.join("|") === fillsHeld.join("|")) return
  fillsHeld = [...next]
  for (const sub of fillSubs) sub()
}

export function useFills(): Fills {
  return useSyncExternalStore(
    sub => {
      fillSubs.add(sub)
      return () => fillSubs.delete(sub)
    },
    () => fillsHeld,
    () => fillsHeld,
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
  set("--border-color", prefs.line === "" ? "" : `var(--c-${prefs.line})`)
  setFills(prefs.fills)
  sound.pref("sound", prefs.sound)
  sound.pref("haptics", prefs.haptics)
  sound.pref("note", prefs.note)
  sound.pref("wave", prefs.wave)
  sound.pref("duration", prefs.duration)
}

let prefsHeld: Prefs = loadPrefs()
const prefSubs = new Set<() => void>()

function keepPrefs(next: Prefs): void {
  prefsHeld = next
  for (const sub of prefSubs) sub()
}

export function usePrefs(): [Prefs, (patch: Partial<Prefs>) => void, () => void] {
  const prefs = useSyncExternalStore(
    sub => {
      prefSubs.add(sub)
      return () => prefSubs.delete(sub)
    },
    () => prefsHeld,
    () => prefsHeld,
  )

  useEffect(() => {
    applyPrefs(prefs)
  }, [prefs])

  const patch = useCallback((part: Partial<Prefs>) => {
    const next = { ...prefsHeld, ...part }
    write(KEY, JSON.stringify(next))
    keepPrefs(next)
  }, [])

  const reset = useCallback(() => {
    write(KEY, "")
    keepPrefs(PREF_DEFAULTS)
  }, [])

  return [prefs, patch, reset]
}
