import { call, setter, pickOpen } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { Shot } from "../../components/Shot.tsx"
import { DESIGNS_VOID, NUMBERS, LEVELS, SUBPIXELS } from "../../components/options.ts"
import { h } from "../../jsx.ts"
import type { Node, Send, Shade } from "../../types.ts"

const SPREADS = ["none", "narrow", "wide", "half", "full"]
const SPINS = ["0", "0.1", "0.3", "0.8", "2"]

type State = {
  settings: {
    design: string
    number: number
    level: number
    padding: number
    subpixel: number
    rays: number
    spread: string
    bounces: number
    spin: number
    accent: string
  }
  play: boolean
  emitters: number
  frame: { rows: number[][]; palette: string[] }
  shade?: Shade
}

const turn = setter("lasers")

export function lasers(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="lasers">
      <card key="board">
        <Board
          app="lasers"
          rows={s.frame.rows}
          palette={s.frame.palette}
          shade={s.shade}
          tap={call("lasers.place")}
        />
      </card>
      <card key="controls">
        <toggle key="play" on={s.play} call={turn("play")} arg="value" label="play" />
        <choice key="design" value={s.settings.design} options={DESIGNS_VOID} call={turn("design")} arg="value" label="design" mode="row" />
        <choice key="number" value={String(s.settings.number)} options={NUMBERS} call={turn("number")} arg="value" label="number" mode="row" />
        <choice key="level" value={String(s.settings.level)} options={LEVELS} call={turn("level")} arg="value" label="level" mode="row" />
        <range key="padding" value={s.settings.padding} min={0} max={48} step={1} call={turn("padding")} arg="value" label="padding" />
        <choice key="spread" value={s.settings.spread} options={SPREADS} call={turn("spread")} arg="value" label="spread" mode="row" />
        <choice key="spin" value={String(s.settings.spin)} options={SPINS} call={turn("spin")} arg="value" label="spin" mode="row" />
        <choice key="subpixel" value={String(s.settings.subpixel)} options={SUBPIXELS} call={turn("subpixel")} arg="value" label="subpixel" mode="row" />
        <button key="accent" call={pickOpen("colors", "lasers", "accent", s.settings.accent)}>{`accent · ${s.settings.accent}`}</button>
        <Shot />
      </card>
      <card key="physics">
        <range key="rays" value={s.settings.rays} min={1} max={128} step={1} call={turn("rays")} arg="value" label="rays" />
        <range key="bounces" value={s.settings.bounces} min={1} max={256} step={1} call={turn("bounces")} arg="value" label="bounces" />
      </card>
      <card key="meter">
        <text key="meter" role="note">{`emitters ${s.emitters}`}</text>
      </card>
    </stack>
  )
}
