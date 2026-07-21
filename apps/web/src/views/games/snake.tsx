import { GameOver } from "../../components/GameOver.tsx"
import { library } from "../../components/library.tsx"
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
    grid: number
    apples: number
    wrap: boolean
    self_collision: boolean
    speed: number
    head: unknown
    body: unknown
    food: unknown
  }
  frame: { rows: number[][]; palette: string[] }
}

export function snake(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="snake">
      <card key="board">
        <Board app="snake" rows={s.frame.rows} palette={s.frame.palette} />
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
        <text key="head-label" role="note">head</text>
        {library("tile", "snake", "head")}
        <text key="body-label" role="note">body</text>
        {library("tile", "snake", "body")}
        <text key="food-label" role="note">food</text>
        {library("tile", "snake", "food")}
      </Section>
    </stack>
  )
}
