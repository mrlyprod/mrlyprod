import { call, setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Pager } from "../../components/Pager.tsx"
import { colors, NUMBERS } from "../../components/options.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const VIEWS = ["iso", "pro", "cut"]

type State = {
  design: string
  index: number
  count: number
  number: number
  level: number
  view: string
  fill: string
  census: { grid: number; fill: number; void: number }
  frame: { rows: number[][]; palette: string[] }
}

const turn = setter("six")

export function six(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="six">
      <card key="board">
        <Board app="six" rows={s.frame.rows} palette={s.frame.palette} />
      </card>
      <card key="page">
        <Pager app="six" current={s.index + 1} total={s.count} />
        <button key="reset" call={call("six.reset")}>reset</button>
        <Shot />
      </card>
      <card key="controls">
        <choice key="number" value={String(s.number)} options={NUMBERS} call={turn("number")} arg="value" label="number" mode="row" />
        <choice key="level" value={String(s.level)} options={["1", "2"]} call={turn("level")} arg="value" label="level" mode="row" />
        <choice key="view" value={s.view} options={VIEWS} call={turn("view")} arg="value" label="view" mode="row" />
        <choice key="fill" value={s.fill} options={colors()} call={turn("fill")} arg="value" label="fill" />
      </card>
      <card key="meter">
        <text key="meter" role="note">{`${s.design} · ${s.census.grid}^3 · fill ${s.census.fill} · void ${s.census.void}`}</text>
      </card>
    </stack>
  )
}
