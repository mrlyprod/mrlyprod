import { call, setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { library } from "../../components/library.tsx"
import { Shot } from "../../components/Shot.tsx"
import { DESIGNS_VOID, NUMBERS, LEVELS, SUBPIXELS } from "../../components/options.ts"
import { h } from "../../jsx.ts"
import type { Node, Send, Shade } from "../../types.ts"

type State = {
  settings: {
    design: string
    number: number
    level: number
    padding: number
    subpixel: number
    speed: number
    trail: number
    size: number
    count: number
    accent: string
  }
  play: boolean
  particles: number
  frame: { rows: number[][]; palette: string[] }
  shade?: Shade
}

const turn = setter("billiards")

export function billiards(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="billiards">
      <card key="board">
        <Board app="billiards" rows={s.frame.rows} palette={s.frame.palette} shade={s.shade} tap={call("billiards.break")} />
      </card>
      <card key="controls">
        <toggle key="play" on={s.play} call={turn("play")} arg="value" label="play" />
        <choice key="design" value={s.settings.design} options={DESIGNS_VOID} call={turn("design")} arg="value" label="design" mode="row" />
        <choice key="number" value={String(s.settings.number)} options={NUMBERS} call={turn("number")} arg="value" label="number" mode="row" />
        <choice key="level" value={String(s.settings.level)} options={LEVELS} call={turn("level")} arg="value" label="level" mode="row" />
        <range key="padding" value={s.settings.padding} min={0} max={48} step={1} call={turn("padding")} arg="value" label="padding" />
        <choice key="subpixel" value={String(s.settings.subpixel)} options={SUBPIXELS} call={turn("subpixel")} arg="value" label="subpixel" mode="row" />
        {library("colors", "billiards", "accent", s.settings.accent)}
        <Shot />
      </card>
      <card key="physics">
        <range key="speed" value={s.settings.speed} min={0.5} max={4.0} call={turn("speed")} arg="value" step={0.1} label="speed" />
        <range key="trail" value={s.settings.trail} min={0.02} max={1.0} call={turn("trail")} arg="value" step={0.02} label="trail" />
        <range key="size" value={s.settings.size} min={1.0} max={4.0} call={turn("size")} arg="value" step={0.5} label="size" />
        <range key="count" value={s.settings.count} min={1} max={128} call={turn("count")} arg="value" step={1} label="count" />
      </card>
      <card key="meter">
        <text key="meter" role="note">{`particles ${s.particles}`}</text>
      </card>
    </stack>
  )
}
