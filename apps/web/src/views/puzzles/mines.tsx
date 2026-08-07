import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Meter } from "../../components/Meter.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { SURFACES, SKINS, DESIGNS_SOLID as DESIGNS } from "../../components/options.ts"
import { call, set } from "../../builders.ts"
import { face, paint, visual } from "../../cells.tsx"
import { h } from "../../jsx.ts"
import type { Cells, Node, Send } from "../../types.ts"

const FLAG = 11

type State = {
  score: number
  steps: number
  over: boolean
  won: boolean | null
  tool: string
  remaining: number
  flags: boolean[][]
  settings: { cols: number; rows: number; mines: number; surface: string; skin: string; design: string }
  cells: Cells
}

function tile(s: State, id: number, flagged: boolean, r: number, c: number): Node {
  const v = visual("mines", s.cells, id)
  if (id === 0) {
    const verb = flagged || s.tool === "flag" ? "mines.flag" : "mines.reveal"
    return (
      <cell key={`c-${r}-${c}`} call={s.over ? undefined : call(verb, { x: c, y: r })} bg={paint(v, s.cells)}>
        {flagged ? face(visual("mines", s.cells, FLAG)) : undefined}
      </cell>
    )
  }
  const worn = face(v)
  if (worn === undefined && v.motif !== undefined) {
    return (
      <cell key={`c-${r}-${c}`}>
        <canvas key="face" handle={`mines-${r}-${c}`} cells={{ app: "mines", ...s.cells, ids: [[id]] }} />
      </cell>
    )
  }
  return (
    <cell key={`c-${r}-${c}`} bg={paint(v, s.cells)}>
      {worn}
    </cell>
  )
}

export function mines(state: unknown, _send: Send): Node {
  const s = state as State
  const grid = s.settings.surface === "grid"
  return (
    <stack key="mines">
      <card key="board">
        {grid
          ? <grid key="grid" cols={s.settings.cols}>
              {s.cells.ids.flatMap((row, r) => row.map((id, c) => tile(s, id, s.flags[r]?.[c] ?? false, r, c)))}
            </grid>
          : <Board app="mines" cells={s.cells} />}
      </card>
      {s.over && <GameOver app="mines" emoji="💣" status={s.won ? `cleared · ${s.score} revealed` : "boom"} />}
      <card key="controls">
        {!s.over && <button key="tool" call={call("mines.tool", { tool: s.tool === "dig" ? "flag" : "dig" })}>{s.tool === "dig" ? "⛏ dig" : "⛳ flag"}</button>}
        <Shot />
      </card>
      {!s.over && <Meter keyName="meter" text={`mines left ${s.remaining} · moves ${s.steps}`} />}
      <Section keyName="rules" label="rules">
        <range key="cols" value={s.settings.cols} min={4} max={30} call={set("mines", "cols")} arg="value" step={1} label="cols" />
        <range key="rows" value={s.settings.rows} min={4} max={30} call={set("mines", "rows")} arg="value" step={1} label="rows" />
        <range key="mines" value={s.settings.mines} min={1} max={200} call={set("mines", "mines")} arg="value" step={1} label="mines" />
      </Section>
      <Section keyName="look" label="look">
        <choice key="surface" value={s.settings.surface} options={SURFACES} call={set("mines", "surface")} arg="value" label="surface" mode="row" />
        <choice key="skin" value={s.settings.skin} options={SKINS} call={set("mines", "skin")} arg="value" label="skin" mode="row" />
        <choice key="design" value={s.settings.design} options={DESIGNS} call={set("mines", "design")} arg="value" label="design" mode="cycle" />
      </Section>
    </stack>
  )
}
