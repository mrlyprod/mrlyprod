import { call } from "../builders.ts"
import { colorpicker } from "./colorpicker.tsx"
import { h } from "../jsx.ts"
import { hex } from "../palette.ts"
import { reading } from "../reads.ts"
import type { Cells, Node } from "../types.ts"

type Slab = { id: number; name: string; value: unknown; cells: Cells }

export function library(kind: "colors" | "emoji" | "font" | "tile", host: string, key: string, current?: unknown): Node[] {
  const lib = reading(`${kind}/library`)
  if (!Array.isArray(lib) || lib.length === 0) return []
  const set = (value: unknown) => call(`${host}.set`, { key, value })
  if (kind === "colors") {
    const swatches = (lib as string[]).map(name => ({ name, hex: hex(name) }))
    return [colorpicker(`lib-${key}`, swatches, name => name === current, set, true)]
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
          <canvas key="thumb" handle={`lib-${host}-${key}-${entry.id}`} cells={{ app: "tile", ...entry.cells }} />
        </cell>
      ))}
    </grid>,
  ]
}
