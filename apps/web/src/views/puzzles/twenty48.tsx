import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Meter } from "../../components/Meter.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { DPad } from "../../components/DPad.tsx"
import { SURFACES, SKINS, DESIGNS_SOLID as DESIGNS } from "../../components/options.ts"
import { set } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const LADDER = ["🌱", "🌿", "🍀", "🌸", "🌼", "🌻", "🍁", "🍄", "🌴", "🌵", "🌟"]

type State = {
  score: number
  steps: number
  over: boolean
  board: number[][]
  last_spawn: [number, number] | null
  last_merges: [number, number][]
  colors: string[]
  settings: { grid: number; surface: string; skin: string; design: string }
  frame: { rows: number[][]; palette: string[] }
}

function exponent(v: number): number {
  let e = 0
  while (v > 1) {
    v >>= 1
    e += 1
  }
  return e
}

function fresh(s: State, r: number, c: number): boolean {
  if (s.last_spawn !== null && s.last_spawn[0] === r && s.last_spawn[1] === c) return true
  return s.last_merges.some(([mr, mc]) => mr === r && mc === c)
}

function face(s: State, v: number, key: string): Node | undefined {
  if (v === 0) return undefined
  const skin = s.settings.skin
  if (skin === "tiles") return undefined
  if (skin === "emojis") {
    const e = exponent(v)
    return <symbol key={key} as="emoji" value={LADDER[Math.min(e - 1, LADDER.length - 1)] ?? "🌟"} />
  }
  return <symbol key={key} as="glyph" value={String(v)} />
}

function tile(s: State, v: number, r: number, c: number): Node {
  const bg: string | undefined = v > 0 && s.settings.skin === "tiles" ? s.colors[exponent(v)] : undefined
  const nonce = fresh(s, r, c) ? `-${s.steps}` : ""
  return (
    <cell key={`t-${r}-${c}`} bg={bg}>
      {face(s, v, `face${nonce}`)}
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
              {s.board.flatMap((row, r) => row.map((v, c) => tile(s, v, r, c)))}
            </grid>
          : <Board app="twenty48" rows={s.frame.rows} palette={s.frame.palette} />}
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
