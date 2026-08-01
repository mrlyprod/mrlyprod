import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Meter } from "../../components/Meter.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { SURFACES, SKINS, DESIGNS_SOLID as DESIGNS } from "../../components/options.ts"
import { call, set } from "../../builders.ts"
import { face, visual, type Skin } from "../../skin.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const VOID = 0
const BACK = 1

type State = {
  score: number
  steps: number
  over: boolean
  rounds: number
  look: number
  ids: number[][]
  matched: boolean[][]
  skin: Skin
  settings: { pairs: number; cols: number; sudden: boolean; surface: string; skin: string; design: string }
  frame: { rows: number[][]; palette: string[] }
}

function tile(s: State, id: number, done: boolean, r: number, c: number): Node {
  const v = visual(s.skin, id)
  if (id === VOID) return <cell key={`k-${r}-${c}`} />
  if (id === BACK) {
    const live = !s.over && s.look === 0
    return <cell key={`k-${r}-${c}`} call={live ? call("memory.flip", { x: c, y: r }) : undefined} bg={v.bg} />
  }
  const key = done ? "m" : `face-${s.steps}`
  return (
    <cell key={`k-${r}-${c}`} on={done} bg={v.bg}>
      {face(v, key)}
    </cell>
  )
}

export function memory(state: unknown, _send: Send): Node {
  const s = state as State
  const grid = s.settings.surface === "grid"
  const status = s.look > 0 ? "look!" : `pairs ${s.score} · round ${s.rounds + 1} · flips ${s.steps}`
  return (
    <stack key="memory">
      <card key="board">
        {grid
          ? <grid key="grid" cols={s.settings.cols}>
              {s.ids.flatMap((row, r) => row.map((id, c) => tile(s, id, s.matched[r]?.[c] ?? false, r, c)))}
            </grid>
          : <Board app="memory" rows={s.frame.rows} palette={s.frame.palette} />}
      </card>
      {s.over && <GameOver app="memory" emoji="🧠" status={`${s.score} pairs · ${s.rounds} rounds`} />}
      <card key="controls">
        <Shot />
      </card>
      {!s.over && <Meter keyName="meter" text={status} />}
      <Section keyName="rules" label="rules">
        <range key="pairs" value={s.settings.pairs} min={2} max={8} call={set("memory", "pairs")} arg="value" step={1} label="pairs" />
        <range key="cols" value={s.settings.cols} min={2} max={8} call={set("memory", "cols")} arg="value" step={1} label="cols" />
        <toggle key="sudden" on={s.settings.sudden} call={set("memory", "sudden")} arg="value" label="sudden death" />
      </Section>
      <Section keyName="look" label="look">
        <choice key="surface" value={s.settings.surface} options={SURFACES} call={set("memory", "surface")} arg="value" label="surface" mode="row" />
        <choice key="skin" value={s.settings.skin} options={SKINS} call={set("memory", "skin")} arg="value" label="skin" mode="row" />
        <choice key="design" value={s.settings.design} options={DESIGNS} call={set("memory", "design")} arg="value" label="design" mode="cycle" />
      </Section>
    </stack>
  )
}
