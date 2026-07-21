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
    speed: number
    damp: number
    freq: number
    sigma: number
    amp: number
    gain: number
    reflect: number
    subpixel: number
    accent: string
    anti: string
  }
  play: boolean
  sources: number
  frame: { rows: number[][]; palette: string[] }
  shade?: Shade
}

const turn = setter("waves")

export function waves(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="waves">
      <card key="board">
        <Board
          app="waves"
          rows={s.frame.rows}
          palette={s.frame.palette}
          shade={s.shade}
          tap={call("waves.drop")}
        />
      </card>
      <card key="controls">
        <toggle key="play" on={s.play} call={turn("play")} arg="value" label="play" />
        <choice key="design" value={s.settings.design} options={DESIGNS_VOID} call={turn("design")} arg="value" label="design" mode="row" />
        <choice key="number" value={String(s.settings.number)} options={NUMBERS} call={turn("number")} arg="value" label="number" mode="row" />
        <choice key="level" value={String(s.settings.level)} options={LEVELS} call={turn("level")} arg="value" label="level" mode="row" />
        <range key="padding" value={s.settings.padding} min={0} max={48} step={1} call={turn("padding")} arg="value" label="padding" />
        <choice key="subpixel" value={String(s.settings.subpixel)} options={SUBPIXELS} call={turn("subpixel")} arg="value" label="subpixel" mode="row" />
        {library("colors", "waves", "accent", s.settings.accent)}
        {library("colors", "waves", "anti", s.settings.anti)}
        <Shot />
      </card>
      <card key="physics">
        <range key="speed" value={s.settings.speed} min={0.05} max={0.45} step={0.01} call={turn("speed")} arg="value" label="speed" />
        <range key="damp" value={s.settings.damp} min={0.0} max={0.02} step={0.001} call={turn("damp")} arg="value" label="damp" />
        <range key="freq" value={s.settings.freq} min={0.3} max={4.0} step={0.1} call={turn("freq")} arg="value" label="freq" />
        <range key="sigma" value={s.settings.sigma} min={1.0} max={6.0} step={0.1} call={turn("sigma")} arg="value" label="sigma" />
        <range key="amp" value={s.settings.amp} min={0.4} max={3.0} step={0.1} call={turn("amp")} arg="value" label="amp" />
        <range key="gain" value={s.settings.gain} min={1} max={16} step={1} call={turn("gain")} arg="value" label="gain" />
        <range key="reflect" value={s.settings.reflect} min={0.0} max={1.0} step={0.05} call={turn("reflect")} arg="value" label="reflect" />
      </card>
      <card key="meter">
        <text key="meter" role="note">{`sources ${s.sources}`}</text>
      </card>
    </stack>
  )
}
