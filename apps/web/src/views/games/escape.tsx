import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Meter } from "../../components/Meter.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { DPad } from "../../components/DPad.tsx"
import { DESIGNS_SOLID as DESIGNS } from "../../components/options.ts"
import { set } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const MAPS = ["random", "0", "1", "2"]

type State = {
  score: number
  steps: number
  level: number
  over: boolean
  escaped: boolean | null
  settings: {
    map: string
    ghost_ratio: number
    speed: number
    design: string
  }
  frame: { rows: number[][]; palette: string[] }
}

export function escape(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="escape">
      <card key="board">
        <Board app="escape" rows={s.frame.rows} palette={s.frame.palette} />
      </card>
      {s.over && <GameOver app="escape" emoji="👻" status={s.escaped ? `escaped · ate ${s.score}` : `caught · level ${s.level}`} />}
      <card key="controls">
        {!s.over && <DPad app="escape" verb="turn" />}
        <Shot />
      </card>
      {!s.over && <Meter text={`level ${s.level} · ate ${s.score}`} />}
      <Section keyName="rules" label="rules">
        <choice key="map" value={s.settings.map} options={MAPS} call={set("escape", "map")} arg="value" label="map" mode="cycle" />
        <range key="ghost_ratio" value={s.settings.ghost_ratio} min={1} max={4} call={set("escape", "ghost_ratio")} arg="value" step={1} label="ghosts" />
      </Section>
      <Section keyName="speed" label="speed">
        <range key="speed" value={s.settings.speed} min={1} max={4} call={set("escape", "speed")} arg="value" step={1} label="speed" />
      </Section>
      <Section keyName="look" label="look">
        <choice key="design" value={s.settings.design} options={DESIGNS} call={set("escape", "design")} arg="value" label="design" mode="cycle" />
      </Section>
    </stack>
  )
}
