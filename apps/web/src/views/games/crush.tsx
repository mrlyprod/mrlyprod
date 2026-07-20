import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Meter } from "../../components/Meter.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { DESIGNS_SOLID as DESIGNS } from "../../components/options.ts"
import { call, set } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  score: number
  steps: number
  over: boolean
  settings: {
    cols: number
    rows: number
    kinds: number
    speed: number
    design: string
  }
  frame: { rows: number[][]; palette: string[] }
}

export function crush(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="crush">
      <card key="board">
        <Board app="crush" rows={s.frame.rows} palette={s.frame.palette} />
      </card>
      {s.over && <GameOver app="crush" emoji="🍬" status={`score ${s.score}`} />}
      <card key="controls">
        {!s.over && [
          <button key="left" call={call("crush.move", { dir: "left" })}>←</button>,
          <button key="right" call={call("crush.move", { dir: "right" })}>→</button>,
          <button key="drop" call={call("crush.drop")}>drop</button>,
          <button key="crush" call={call("crush.crush")}>crush</button>,
        ]}
        <Shot />
      </card>
      {!s.over && <Meter text={`score ${s.score} · steps ${s.steps}`} />}
      <Section keyName="rules" label="rules">
        <range key="cols" value={s.settings.cols} min={4} max={16} call={set("crush", "cols")} arg="value" step={1} label="cols" />
        <range key="rows" value={s.settings.rows} min={4} max={16} call={set("crush", "rows")} arg="value" step={1} label="rows" />
        <range key="kinds" value={s.settings.kinds} min={2} max={8} call={set("crush", "kinds")} arg="value" step={1} label="kinds" />
      </Section>
      <Section keyName="speed" label="speed">
        <range key="speed" value={s.settings.speed} min={1} max={8} call={set("crush", "speed")} arg="value" step={1} label="speed" />
      </Section>
      <Section keyName="look" label="look">
        <choice key="design" value={s.settings.design} options={DESIGNS} call={set("crush", "design")} arg="value" label="design" mode="cycle" />
      </Section>
    </stack>
  )
}
