import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Meter } from "../../components/Meter.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { DPad } from "../../components/DPad.tsx"
import { SURFACES, SKINS, DESIGNS_SOLID as DESIGNS } from "../../components/options.ts"
import { set } from "../../builders.ts"
import { face, visual, type Skin } from "../../skin.tsx"
import { h } from "../../jsx.ts"
import type { Glyph, Node, Send } from "../../types.ts"

type State = {
  score: number
  steps: number
  over: boolean
  ids: number[][]
  last_spawn: [number, number] | null
  last_merges: [number, number][]
  skin: Skin
  settings: { grid: number; surface: string; skin: string; design: string }
  frame: { rows: number[][]; palette: string[]; glyphs?: Glyph[] }
}

function fresh(s: State, r: number, c: number): boolean {
  if (s.last_spawn !== null && s.last_spawn[0] === r && s.last_spawn[1] === c) return true
  return s.last_merges.some(([mr, mc]) => mr === r && mc === c)
}

function tile(s: State, id: number, r: number, c: number): Node {
  const v = visual(s.skin, id)
  const nonce = fresh(s, r, c) ? `-${s.steps}` : ""
  return (
    <cell key={`t-${r}-${c}`} bg={v.bg}>
      {face(v, `face${nonce}`)}
    </cell>
  )
}

export function twenty48(state: unknown, _send: Send): Node {
  const s = state as State
  const grid = s.settings.surface === "grid"
  return (
    <stack key="twenty48">
      <card key="board">
        {grid
          ? <grid key="grid" cols={s.settings.grid}>
              {s.ids.flatMap((row, r) => row.map((id, c) => tile(s, id, r, c)))}
            </grid>
          : <Board app="twenty48" rows={s.frame.rows} palette={s.frame.palette} glyphs={s.frame.glyphs} />}
      </card>
      {s.over && <GameOver app="twenty48" emoji="🔢" status={`score ${s.score}`} />}
      <card key="controls">
        {!s.over && <DPad app="twenty48" verb="slide" />}
        <Shot />
      </card>
      {!s.over && <Meter text={`score ${s.score} · steps ${s.steps}`} />}
      <Section keyName="rules" label="rules">
        <range key="grid" value={s.settings.grid} min={2} max={8} call={set("twenty48", "grid")} arg="value" step={1} label="grid" />
      </Section>
      <Section keyName="look" label="look">
        <choice key="surface" value={s.settings.surface} options={SURFACES} call={set("twenty48", "surface")} arg="value" label="surface" mode="row" />
        <choice key="skin" value={s.settings.skin} options={SKINS} call={set("twenty48", "skin")} arg="value" label="skin" mode="row" />
        <choice key="design" value={s.settings.design} options={DESIGNS} call={set("twenty48", "design")} arg="value" label="design" mode="cycle" />
      </Section>
    </stack>
  )
}
