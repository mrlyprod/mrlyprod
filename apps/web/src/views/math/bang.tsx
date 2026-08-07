import { call } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { Pager } from "../../components/Pager.tsx"
import { Shot } from "../../components/Shot.tsx"
import { h } from "../../jsx.ts"
import { orbit } from "../../render/orbit.ts"
import type { Cells, Node, Send, Shade } from "../../types.ts"

const DIMENSIONS = ["1", "2", "3"]

type State = {
  dimension: number
  base: number
  index: number
  count: number
  name: string
  code: string
  degree: number
  anf: string
  cells?: Cells
  shade?: Shade
}

export function bang(state: unknown, _send: Send): Node {
  const s = state as State
  const solid = s.dimension === 3
  return (
    <stack key="bang">
      <card key="board">
        {solid ? (
          <Board
            app="bang"
            shade={s.shade}
            turn={call("orbit.turn", { app: "bang" })}
            zoom={call("orbit.zoom", { app: "bang" })}
            pan={call("orbit.pan", { app: "bang" })}
          />
        ) : (
          <Board app="bang" cells={s.cells} />
        )}
      </card>
      <card key="page">
        <Pager app="bang" />
        <button key="reset" call={call("bang.reset")}>reset</button>
        <Shot />
      </card>
      <card key="controls">
        <choice key="dimension" value={String(s.dimension)} options={DIMENSIONS} call={call("bang.set", { key: "dimension" })} arg="value" label="dimension" mode="row" />
        {solid && <toggle key="ortho" on={orbit("bang").ortho} call={call("orbit.ortho", { app: "bang" })} arg="value" label="ortho" />}
      </card>
      <card key="facts">
        <text key="name" role="note">{s.name}</text>
        <text key="code" role="note">{`code ${s.code} · degree ${s.degree} · anf ${s.anf}`}</text>
        <text key="position" role="note">{`${s.index + 1} / ${s.count}`}</text>
      </card>
    </stack>
  )
}
