import { h } from "../jsx.ts"
import type { Call, Node } from "../types.ts"

export function colorpicker(gridKey: string, swatches: { name: string; hex: string }[], on: (name: string) => boolean, pick: (name: string) => Call, big?: boolean): Node {
  return (
    <grid key={gridKey} cols={5}>
      {swatches.map(s => (
        <button key={`${gridKey}-${s.name}`} bg={s.hex} big={big} call={pick(s.name)}>{on(s.name) ? "✓" : " "}</button>
      ))}
    </grid>
  )
}
