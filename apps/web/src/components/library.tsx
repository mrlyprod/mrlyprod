import { call } from "../builders.ts"
import { h } from "../jsx.ts"
import { hex } from "../palette.ts"
import { peeked } from "../peeks.ts"
import type { Node } from "../types.ts"

type Frame = { width: number; height: number; rows: number[][]; palette: string[] }

type Slab = { id: number; name: string; value: unknown; frame: Frame }

export function library(kind: "colors" | "emoji" | "font" | "tile", host: string, key: string, current?: unknown): Node[] {
  const state = peeked(kind)?.state as { library?: unknown } | undefined
  const lib = state?.library
  if (!Array.isArray(lib) || lib.length === 0) return []
  const set = (value: unknown) => call(`${host}.set`, { key, value })
  if (kind === "colors") {
    return [
      <grid key={`lib-${key}`} cols={5}>
        {(lib as string[]).map(name => (
          <button key={`${key}-${name}`} bg={hex(name)} call={set(name)}>{name === current ? "✓" : " "}</button>
        ))}
      </grid>,
    ]
  }
  if (kind === "emoji" || kind === "font") {
    return [
      <grid key={`lib-${key}`} cols={8}>
        {(lib as string[]).map(value => (
          <button key={`${key}-${value}`} call={set(value)}>{value}</button>
        ))}
      </grid>,
    ]
  }
  return [
    <grid key={`lib-${key}`} cols={3}>
      {(lib as Slab[]).map(entry => (
        <cell key={`${key}-${entry.id}`} call={set(entry.value)}>
          <canvas key="thumb" handle={`lib-${host}-${key}-${entry.id}`} rows={entry.frame.rows} palette={entry.frame.palette} />
        </cell>
      ))}
    </grid>,
  ]
}
