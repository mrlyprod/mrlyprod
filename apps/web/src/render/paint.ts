import { pool } from "../palette.ts"
import type { Held } from "../types.ts"

let pinned: string | null = null

function pick(): string {
  const options = pool()
  return options[Math.floor(Math.random() * options.length)] as string
}

export function pin(value: string | null): void {
  pinned = value
}

export function paint(el: Held): string {
  if (pinned !== null) return `var(--c-${pinned})`
  el.__tint ??= pick()
  return `var(--c-${el.__tint})`
}

export function tint(prior?: string): string {
  if (pinned !== null) return pinned
  return prior ?? pick()
}

export function make(tag: string, cls: string): Held {
  const el = document.createElement(tag) as Held
  el.className = cls
  return el
}
