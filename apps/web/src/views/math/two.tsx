import { call, setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Pager } from "../../components/Pager.tsx"
import { library } from "../../components/library.tsx"
import { colors, MODES, NUMBERS, LEVELS } from "../../components/options.ts"
import { visual, type Skin } from "../../skin.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  settings: { design: string; number: number; level: number; skin: string; fill: string; void: string }
  index: number
  count: number
  census: { grid: number; fill: number; void: number }
  skin: Skin
  ids: number[][] | null
  frame: { rows: number[][]; palette: string[] } | null
}

const turn = setter("two")

function board(s: State): Node {
  if (s.ids !== null) {
    const rows = s.ids.map(row => row.map(id => visual(s.skin, id).face?.value ?? " "))
    return <cells key="grid" rows={rows} />
  }
  return <Board app="two" rows={s.frame?.rows ?? []} palette={s.frame?.palette} />
}

function paints(s: State): Node[] {
  if (s.settings.skin === "glyphs") {
    return [
      <field key="fill" value={s.settings.fill} live={false} call={turn("fill")} arg="value" label="fill" />,
      ...library("emoji", "two", "fill", s.settings.fill),
      <field key="void" value={s.settings.void} live={false} call={turn("void")} arg="value" label="void" />,
      ...library("font", "two", "void", s.settings.void),
    ]
  }
  return [
    <choice key="fill" value={s.settings.fill} options={colors()} call={turn("fill")} arg="value" label="fill" />,
    <choice key="void" value={s.settings.void} options={colors()} call={turn("void")} arg="value" label="void" />,
  ]
}

export function two(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="two">
      <card key="board">{board(s)}</card>
      <card key="page">
        <Pager app="two" current={s.index + 1} total={s.count} />
        <button key="reset" call={call("two.reset")}>reset</button>
        {s.frame !== null && <Shot />}
      </card>
      <card key="controls">
        <choice key="number" value={String(s.settings.number)} options={NUMBERS} call={turn("number")} arg="value" label="number" mode="row" />
        <choice key="level" value={String(s.settings.level)} options={LEVELS} call={turn("level")} arg="value" label="level" mode="row" />
        <choice key="skin" value={s.settings.skin} options={MODES} call={turn("skin")} arg="value" label="skin" mode="row" />
        {paints(s)}
      </card>
      <card key="meter">
        <text key="meter" role="note">{`${s.settings.design} · ${s.census.grid}x${s.census.grid} · fill ${s.census.fill} · void ${s.census.void}`}</text>
      </card>
    </stack>
  )
}
