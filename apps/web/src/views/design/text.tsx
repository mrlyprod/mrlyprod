import { call, setter, pickOpen } from "../../builders.ts"
import { Pager } from "../../components/Pager.tsx"
import { DESIGNS_VOID, NUMBERS, LEVELS } from "../../components/options.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  design: string
  number: number
  level: number
  fill: string
  void: string
  cols: number
  rows: number
  grid: string[]
}

const turn = setter("text")

export function text(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="text">
      <card key="board">
        <cells key="grid" rows={s.grid.map(row => Array.from(row))} />
      </card>
      <card key="page">
        <Pager app="text" current={DESIGNS_VOID.indexOf(s.design) + 1} total={DESIGNS_VOID.length} />
        <button key="reset" call={call("text.reset")}>reset</button>
      </card>
      <card key="controls">
        <choice key="design" value={s.design} options={DESIGNS_VOID} call={turn("design")} arg="value" label="design" mode="row" />
        <choice key="number" value={String(s.number)} options={NUMBERS} call={turn("number")} arg="value" label="number" mode="row" />
        <choice key="level" value={String(s.level)} options={LEVELS} call={turn("level")} arg="value" label="level" mode="row" />
        <field key="fill" value={s.fill} live={false} call={turn("fill")} arg="value" label="fill" />
        <button key="fill-pick" call={pickOpen("emoji", "text", "fill", s.fill)}>pick fill</button>
        <field key="void" value={s.void} live={false} call={turn("void")} arg="value" label="void" />
        <button key="void-pick" call={pickOpen("font", "text", "void", s.void)}>pick void</button>
      </card>
      <card key="facts">
        <text key="size" role="note">{`${s.cols}x${s.rows}`}</text>
        <text key="chars" role="note">{`fill ${s.fill} · void ${s.void}`}</text>
      </card>
    </stack>
  )
}
