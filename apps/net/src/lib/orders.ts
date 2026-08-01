import { read, write } from "mrlydom"
import { BASKET } from "./site"

// SHAPE

export type Order = {
  product: string
  tier: string
  label: string
  price: number
  config: Record<string, string | number | boolean>
  at: number
}

function valid(value: unknown): value is Order {
  if (value === null || typeof value !== "object") return false
  const row = value as Partial<Order>
  return typeof row.product === "string" && typeof row.label === "string" && typeof row.price === "number"
}

function parse(): Order[] {
  let raw: unknown = null
  try {
    raw = JSON.parse(read(BASKET) || "[]")
  } catch {}
  return Array.isArray(raw) ? raw.filter(valid) : []
}

// STORE

let held = parse()

const watchers = new Set<() => void>()

function save(next: Order[]): void {
  held = next
  write(BASKET, JSON.stringify(next))
  for (const fire of watchers) fire()
}

export function listen(fire: () => void): () => void {
  watchers.add(fire)
  return () => {
    watchers.delete(fire)
  }
}

export function snapshot(): Order[] {
  return held
}

export function place(order: Order): void {
  save([...parse(), order])
}

export function drop(at: number): void {
  save(parse().filter((_, i) => i !== at))
}
