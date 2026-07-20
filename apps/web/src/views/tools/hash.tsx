import { call, setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { Shot } from "../../components/Shot.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const RULES = ["life", "maze", "replicator", "anneal"]

type State = {
  text: string
  hex: string
  rule: string
  frame: { rows: number[][]; palette: string[] }
}

const turn = setter("hash")

export function hash(state: unknown, _send: Send): Node {
  const s = state as State
  const hex = s.hex.length > 32 ? `${s.hex.slice(0, 32)}…` : s.hex
  return (
    <stack key="hash">
      <card key="board">
        <Board app="hash" rows={s.frame.rows} palette={s.frame.palette} />
      </card>
      <card key="digest">
        <field key="text" value={s.text} live={false} call={call("hash.digest")} arg="text" label="text" />
        <button key="go" call={call("hash.digest", { text: s.text })}>digest</button>
        <choice key="rule" value={s.rule} options={RULES} call={turn("rule")} arg="value" label="rule" mode="row" />
        <button key="reset" call={call("hash.reset")}>reset</button>
        <Shot />
      </card>
      <card key="facts">
        <text key="hex" role="note">{hex}</text>
      </card>
    </stack>
  )
}
