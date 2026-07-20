import { call } from "../../builders.ts"
import { Meter } from "../../components/Meter.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type Doc = { name: string; mime: string; uri: string; size: number; tick: number }

type State = { count: number; files: Doc[] }

export function files(state: unknown, _send: Send): Node {
  const s = state as State
  const meter = s.count === 0 ? "no documents yet" : `${s.count} on the shelf`
  const rows = s.files.map((f, i) => (
    <card key={`file-${i}`}>
      <label key={`file-${i}-open`} mode="text" text={f.name} note={`${f.size} bytes`} href={f.uri} />
      <button key={`file-${i}-drop`} call={call("files.drop", { index: i })}>✕</button>
    </card>
  ))
  return (
    <stack key="files">
      {rows}
      <card key="footer">
        <button key="clear" call={call("files.clear")}>clear all</button>
      </card>
      <Meter keyName="meter" text={meter} />
    </stack>
  )
}
