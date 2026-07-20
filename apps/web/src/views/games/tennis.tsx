import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Meter } from "../../components/Meter.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { DPad } from "../../components/DPad.tsx"
import { set } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  score: number
  steps: number
  over: boolean
  settings: {
    board: number
    paddle: number
    block: number
    rows: number
    physics: number
    speed: number
  }
  frame: { rows: number[][]; palette: string[] }
}

export function tennis(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="tennis">
      <card key="board">
        <Board app="tennis" rows={s.frame.rows} palette={s.frame.palette} />
      </card>
      {s.over && <GameOver app="tennis" emoji="🎾" status={`score ${s.score}`} />}
      <card key="controls">
        {!s.over && <DPad app="tennis" verb="move" />}
        <Shot />
      </card>
      {!s.over && <Meter text={`score ${s.score} · steps ${s.steps}`} />}
      <Section keyName="rules" label="rules">
        <range key="board" value={s.settings.board} min={8} max={40} call={set("tennis", "board")} arg="value" step={1} label="board" />
        <range key="paddle" value={s.settings.paddle} min={2} max={10} call={set("tennis", "paddle")} arg="value" step={1} label="paddle" />
        <range key="block" value={s.settings.block} min={1} max={6} call={set("tennis", "block")} arg="value" step={1} label="block" />
        <range key="rows" value={s.settings.rows} min={1} max={10} call={set("tennis", "rows")} arg="value" step={1} label="rows" />
        <range key="physics" value={s.settings.physics} min={0.1} max={0.9} call={set("tennis", "physics")} arg="value" step={0.1} label="physics" />
      </Section>
      <Section keyName="speed" label="speed">
        <range key="speed" value={s.settings.speed} min={1} max={8} call={set("tennis", "speed")} arg="value" step={1} label="speed" />
      </Section>
    </stack>
  )
}
