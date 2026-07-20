import { call } from "../builders.ts"
import { h } from "../jsx.ts"
import type { Node } from "../types.ts"

export function Pager({ app, verb = "page", current, total }: { app: string; verb?: string; current?: number; total?: number }): Node {
  return (
    <grid key="pager" cols={3}>
      <button key="prev" call={call(`${app}.${verb}`, { dir: "prev" })}>←</button>
      {current !== undefined && total !== undefined && <text key="counter" role="note">{`${current} / ${total}`}</text>}
      <button key="next" call={call(`${app}.${verb}`, { dir: "next" })}>→</button>
    </grid>
  )
}
