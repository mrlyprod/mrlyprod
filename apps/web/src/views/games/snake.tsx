import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Meter } from "../../components/Meter.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { DPad } from "../../components/DPad.tsx"
import { DESIGNS_SOLID as DESIGNS } from "../../components/options.ts"
import { set } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Cells, Node, Send } from "../../types.ts"

type State = {
  score: number
  steps: number
  over: boolean
  settings: {
    grid: number
    apples: number
    wrap: boolean
    self_collision: boolean
    speed: number
    design: string
  }
  cells: Cells
}

export function snake(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="snake">
      <card key="board">
        <Board app="snake" cells={s.cells} />
      </card>
      {s.over && <GameOver app="snake" emoji="🐍" status={`score ${s.score}`} />}
      <card key="controls">
        {!s.over && <DPad app="snake" verb="turn" />}
        <Shot />
      </card>
      {!s.over && <Meter text={`score ${s.score} · steps ${s.steps}`} />}
      <Section keyName="rules" label="rules">
        <range key="grid" value={s.settings.grid} min={5} max={64} call={set("snake", "grid")} arg="value" step={1} label="grid" />
        <range key="apples" value={s.settings.apples} min={1} max={16} call={set("snake", "apples")} arg="value" step={1} label="apples" />
        <toggle key="wrap" on={s.settings.wrap} call={set("snake", "wrap")} arg="value" label="wrap" />
        <toggle key="self_collision" on={s.settings.self_collision} call={set("snake", "self_collision")} arg="value" label="self collision" />
      </Section>
      <Section keyName="speed" label="speed">
        <range key="speed" value={s.settings.speed} min={1} max={8} call={set("snake", "speed")} arg="value" step={1} label="speed" />
      </Section>
      <Section keyName="look" label="look">
        <choice key="design" value={s.settings.design} options={DESIGNS} call={set("snake", "design")} arg="value" label="design" mode="cycle" />
      </Section>
    </stack>
  )
}
