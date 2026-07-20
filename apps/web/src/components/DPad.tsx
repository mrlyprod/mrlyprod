import { call } from "../builders.ts"
import { h } from "../jsx.ts"
import type { Node } from "../types.ts"

export function DPad({ app, verb }: { app: string; verb: string }): Node {
  return (
    <grid key="dpad" cols={4}>
      <button key="left" call={call(`${app}.${verb}`, { dir: "left" })}>←</button>
      <button key="up" call={call(`${app}.${verb}`, { dir: "up" })}>↑</button>
      <button key="down" call={call(`${app}.${verb}`, { dir: "down" })}>↓</button>
      <button key="right" call={call(`${app}.${verb}`, { dir: "right" })}>→</button>
    </grid>
  )
}
