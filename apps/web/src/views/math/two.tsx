import { call, setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Pager } from "../../components/Pager.tsx"
import { colors, NUMBERS, LEVELS } from "../../components/options.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  design: string
  index: number
  count: number
  number: number
  level: number
  fill: string
  void: string
  census: { grid: number; fill: number; void: number }
  frame: { rows: number[][]; palette: string[] }
}

const turn = setter("two")

export function two(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="two">
      <card key="board">
        <Board app="two" rows={s.frame.rows} palette={s.frame.palette} />
      </card>
      <card key="page">
        <Pager app="two" current={s.index + 1} total={s.count} />
        <button key="reset" call={call("two.reset")}>reset</button>
        <Shot />
      </card>
      <card key="controls">
        <choice key="number" value={String(s.number)} options={NUMBERS} call={turn("number")} arg="value" label="number" mode="row" />
        <choice key="level" value={String(s.level)} options={LEVELS} call={turn("level")} arg="value" label="level" mode="row" />
        <choice key="fill" value={s.fill} options={colors()} call={turn("fill")} arg="value" label="fill" />
        <choice key="void" value={s.void} options={colors()} call={turn("void")} arg="value" label="void" />
      </card>
      <card key="meter">
        <text key="meter" role="note">{`${s.design} · ${s.census.grid}x${s.census.grid} · fill ${s.census.fill} · void ${s.census.void}`}</text>
      </card>
    </stack>
  )
}
