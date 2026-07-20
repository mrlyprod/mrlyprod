import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Meter } from "../../components/Meter.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { SURFACES, SKINS, DESIGNS_SOLID as DESIGNS } from "../../components/options.ts"
import { call, set } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  score: number
  steps: number
  over: boolean
  won: boolean | null
  tool: string
  remaining: number
  board: (number | string | null)[][]
  flags: boolean[][]
  colors: string[]
  hidden: string
  mine: string
  settings: { cols: number; rows: number; mines: number; surface: string; skin: string; design: string }
  frame: { rows: number[][]; palette: string[] }
}

function face(s: State, cell: number | string): Node | undefined {
  const skin = s.settings.skin
  if (cell === "mine") {
    return skin === "emojis" ? <symbol key="face" as="emoji" value="💣" /> : undefined
  }
  const n = cell as number
  if (n === 0 || skin === "tiles") return undefined
  return <symbol key="face" as="glyph" value={String(n)} />
}

function tile(s: State, cell: number | string | null, flagged: boolean, r: number, c: number): Node {
  if (cell === null) {
    const verb = flagged || s.tool === "flag" ? "mines.flag" : "mines.reveal"
    return (
      <cell key={`c-${r}-${c}`} call={s.over ? undefined : call(verb, { x: c, y: r })} bg={s.hidden}>
        {flagged ? <symbol key="face" as="emoji" value="⛳" /> : undefined}
      </cell>
    )
  }
  const bg: string | undefined = cell === "mine" ? s.mine : s.settings.skin === "tiles" ? s.colors[cell as number] : undefined
  return (
    <cell key={`c-${r}-${c}`} bg={bg}>
      {face(s, cell)}
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
              {s.board.flatMap((row, r) => row.map((cell, c) => tile(s, cell, s.flags[r]?.[c] ?? false, r, c)))}
            </grid>
          : <Board app="mines" rows={s.frame.rows} palette={s.frame.palette} />}
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
