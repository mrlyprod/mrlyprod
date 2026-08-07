import type { Call, View } from "./types"

const WORDS = ["moon", "ferry", "jazz", "milk", "totem"]
const FILES = ["a", "b", "c", "d", "e", "f", "g", "h"]

function between(lo: number, hi: number): number {
  return lo + Math.floor(Math.random() * (hi - lo + 1))
}

function roll(key: string, hint: string): unknown {
  if (hint.includes("|")) {
    const options = hint.split("|").map(o => o.trim())
    return options[Math.floor(Math.random() * options.length)]
  }
  const range = /^int (\d+)\.\.(\d+)$/.exec(hint)
  if (range !== null) return between(Number(range[1]), Number(range[2]))
  if (hint === "int" || hint === "number" || hint === "u8" || hint === "u64") {
    return key === "seed" ? between(0, 999999999) : between(0, 15)
  }
  if (hint === "bool") return Math.random() < 0.5
  if (hint === "square") return `${FILES[between(0, 7)]}${between(1, 8)}`
  if (hint === "string" || hint === "text") return WORDS[between(0, WORDS.length - 1)]
  return undefined
}

export function improvise(view: View | null | undefined): Call | null {
  const actions = view?.actions ?? []
  const legal: Call[] = []
  for (const action of actions) {
    const fields = Object.entries((action.args ?? {}) as Record<string, unknown>)
    const args: Record<string, unknown> = {}
    let filled = true
    for (const [key, hint] of fields) {
      const value = typeof hint === "string" ? roll(key, hint) : undefined
      if (value === undefined) {
        filled = false
        break
      }
      args[key] = value
    }
    if (filled) legal.push({ verb: action.verb, args })
  }
  if (legal.length === 0) return null
  return legal[Math.floor(Math.random() * legal.length)] ?? null
}
