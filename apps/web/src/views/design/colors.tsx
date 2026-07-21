import { call } from "../../builders.ts"
import { h } from "../../jsx.ts"
import { hex } from "../../palette.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  index: number
  count: number
  name: string
  hex: string
  rgb: { r: number; g: number; b: number }
  palette: { name: string; hex: string }[]
  library: string[]
}

export function colors(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="colors">
      <card key="browse">
        <button key="active" call={call("colors.page", { dir: "next" })} bg={s.hex} big={true}></button>
        <grid key="palette" cols={5}>
          {s.palette.map(p => (
            <button key={`swatch-${p.name}`} call={call("colors.set", { key: "name", value: p.name })} bg={p.hex}>{p.name === s.name ? "✓" : ""}</button>
          ))}
        </grid>
        <button key="keep" call={call("colors.keep")}>keep</button>
      </card>
      <card key="facts">
        <text key="name" role="label">{s.name}</text>
        <text key="hex">{s.hex}</text>
        <text key="rgb" role="note">{`${s.rgb.r} ${s.rgb.g} ${s.rgb.b}`}</text>
      </card>
      <card key="export">
        <button key="export" call={call("colors.export")}>export</button>
      </card>
      <card key="library">
        <text key="drop-hint" role="note">tap to drop</text>
        <grid key="lib" cols={5}>
          {s.library.map(name => (
            <button key={`lib-${name}`} bg={hex(name)} call={call("colors.drop", { name })}>{" "}</button>
          ))}
        </grid>
      </card>
    </stack>
  )
}
