import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = { handle: string; id: string; verified: boolean }

export function iden(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="iden">
      <card key="who">
        <text key="guest" role="title">Guest</text>
        <text key="handle" role="note">{s.handle}</text>
      </card>
    </stack>
  )
}
