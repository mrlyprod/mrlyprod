import { call, setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Pager } from "../../components/Pager.tsx"
import { colors, NUMBERS, LEVELS } from "../../components/options.ts"
import { h } from "../../jsx.ts"
import type { Cells, Node, Send } from "../../types.ts"

type State = {
  settings: { design: string; number: number; level: number; fill: string; void: string }
  index: number
  count: number
  census: { grid: number; fill: number; void: number }
  cells: Cells
}

const turn = setter("two")

export function two(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="two">
      <card key="board">
        <Board app="two" cells={s.cells} />
      </card>
      <card key="page">
        <Pager app="two" current={s.index + 1} total={s.count} />
        <button key="reset" call={call("two.reset")}>reset</button>
        <Shot />
      </card>
      <card key="controls">
        <choice key="number" value={String(s.settings.number)} options={NUMBERS} call={turn("number")} arg="value" label="number" mode="row" />
        <choice key="level" value={String(s.settings.level)} options={LEVELS} call={turn("level")} arg="value" label="level" mode="row" />
        <choice key="fill" value={s.settings.fill} options={colors()} call={turn("fill")} arg="value" label="fill" />
        <choice key="void" value={s.settings.void} options={colors()} call={turn("void")} arg="value" label="void" />
      </card>
      <card key="meter">
        <text key="meter" role="note">{`${s.settings.design} · ${s.census.grid}x${s.census.grid} · fill ${s.census.fill} · void ${s.census.void}`}</text>
      </card>
    </stack>
  )
}
