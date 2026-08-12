import type { Args, Call, Verb, View } from "./types"

const WORDS = ["moon", "ferry", "jazz", "milk", "totem"]
const FILES = ["a", "b", "c", "d", "e", "f", "g", "h"]
const TOKENS = ["int", "number", "u8", "u64", "bool", "square", "string", "text"]
const STROKE = 4
const WHOLE = /^-?\d+$/
const RANGE = /^int (-?\d+)\.\.(-?\d+)$/
const POINTS = /^points (-?\d+)\.\.(-?\d+) (-?\d+)\.\.(-?\d+)$/

function between(lo: number, hi: number): number {
  return lo + Math.floor(Math.random() * (hi - lo + 1))
}

function fields(hint: unknown): Args | null {
  if (typeof hint !== "object" || hint === null || Array.isArray(hint)) return null
  return hint as Args
}

function menu(args: Args): { target: string; of: string } | null {
  for (const [name, hint] of Object.entries(args)) {
    if (typeof hint !== "string" || !hint.startsWith("of ")) continue
    return { target: hint.slice(3).trim(), of: name }
  }
  return null
}

function span(found: RegExpExecArray | null, at: number): [number, number] | null {
  if (found === null) return null
  const lo = Number(found[at])
  const hi = Number(found[at + 1])
  return hi < lo ? null : [lo, hi]
}

function reads(token: string): boolean {
  if (token.includes("|")) return true
  if (token.startsWith("points ")) {
    const found = POINTS.exec(token)
    return span(found, 1) !== null && span(found, 3) !== null
  }
  if (token.startsWith("int ")) return span(RANGE.exec(token), 1) !== null
  return TOKENS.includes(token)
}

function fillable(hint: unknown): boolean {
  if (typeof hint === "string") return reads(hint)
  const inner = fields(hint)
  if (inner === null) return false
  const values = Object.values(inner)
  return values.length > 0 && values.every(fillable)
}

function roll(key: string, hint: string): unknown {
  if (hint.includes("|")) {
    const options = hint.split("|").map(o => o.trim())
    const chosen = options[Math.floor(Math.random() * options.length)] ?? ""
    return options.every(o => WHOLE.test(o)) ? Number(chosen) : chosen
  }
  if (hint.startsWith("points ")) {
    const found = POINTS.exec(hint)
    const x = span(found, 1)
    const y = span(found, 3)
    if (x === null || y === null) return undefined
    const out: number[][] = []
    for (let left = between(1, STROKE); left > 0; left -= 1) {
      out.push([between(x[0], x[1]), between(y[0], y[1])])
    }
    return out
  }
  if (hint.startsWith("int ")) {
    const range = span(RANGE.exec(hint), 1)
    return range === null ? undefined : between(range[0], range[1])
  }
  if (hint === "int" || hint === "number" || hint === "u8" || hint === "u64") {
    return key === "seed" ? between(0, 999999999) : between(0, 15)
  }
  if (hint === "bool") return Math.random() < 0.5
  if (hint === "square") return `${FILES[between(0, 7)]}${between(1, 8)}`
  if (hint === "string" || hint === "text") return WORDS[between(0, WORDS.length - 1)]
  return undefined
}

function grow(key: string, hint: unknown): unknown {
  if (typeof hint === "string") return roll(key, hint)
  const inner = fields(hint)
  if (inner === null) return undefined
  const entries = Object.entries(inner)
  if (entries.length === 0) return undefined
  const out: Args = {}
  for (const [name, leaf] of entries) {
    const value = grow(name, leaf)
    if (value === undefined) return undefined
    out[name] = value
  }
  return out
}

function fill(args: Args): Args | null {
  const picked = menu(args)
  const out: Args = {}
  let chosen: unknown
  for (const [key, hint] of Object.entries(args)) {
    if (picked !== null) {
      if (key === picked.of) continue
      if (key === picked.target) {
        const inner = fields(hint)
        const names = inner === null ? [] : Object.keys(inner)
        const name = names[between(0, names.length - 1)]
        if (inner === null || name === undefined) return null
        out[key] = name
        chosen = inner[name]
        continue
      }
    }
    const value = grow(key, hint)
    if (value === undefined) return null
    out[key] = value
  }
  if (picked !== null) {
    const value = grow(picked.of, chosen)
    if (value === undefined) return null
    out[picked.of] = value
  }
  return out
}

export function plays(verb: Verb): boolean {
  const args = verb.args ?? {}
  const picked = menu(args)
  if (picked !== null) {
    const target = args[picked.target]
    if (fields(target) === null || !fillable(target)) return false
  }
  return Object.entries(args).every(([key, hint]) => {
    if (picked !== null && (key === picked.of || key === picked.target)) return true
    return fillable(hint)
  })
}

export function improvise(view: View | null | undefined): Call | null {
  const actions = view?.actions ?? []
  const legal: Call[] = []
  for (const action of actions) {
    if (!plays(action)) continue
    const args = fill(action.args ?? {})
    if (args !== null) legal.push({ verb: action.verb, args })
  }
  if (legal.length === 0) return null
  return legal[Math.floor(Math.random() * legal.length)] ?? null
}
