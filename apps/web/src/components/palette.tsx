import { call } from "../builders.ts"
import { h } from "../jsx.ts"
import { hex, names } from "../palette.ts"
import { peeked } from "../peeks.ts"
import type { Node } from "../types.ts"

export function palette(app: string, colors: string[]): Node[] {
  const lib = (peeked("colors")?.state as { library?: string[] } | undefined)?.library ?? []
  const pool = lib.length > 0 ? lib : names()
  const byHex = new Map(names().map(name => [hex(name).toLowerCase(), name]))
  const cycle = (slot: string) => {
    const at = pool.indexOf(byHex.get(slot.toLowerCase()) ?? "")
    return pool[(at + 1) % pool.length] ?? pool[0]
  }
  return [
    ...colors.map((slot, i) => (
      <button key={`slot-${i}`} bg={slot} call={call(`${app}.set`, { key: `palette.${i}`, value: cycle(slot) })}>{" "}</button>
    )),
    ...(colors.length > 1
      ? colors.map((_, i) => (
          <button key={`drop-${i}`} call={call(`${app}.set`, { key: "palette", value: colors.filter((_, j) => j !== i) })}>{`× ${i + 1}`}</button>
        ))
      : []),
    <button key="add" call={call(`${app}.set`, { key: "palette", value: [...colors, colors[colors.length - 1] ?? "#ffffff"] })}>+</button>,
  ]
}
