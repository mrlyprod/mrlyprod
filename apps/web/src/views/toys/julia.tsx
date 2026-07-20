import { setter } from "../../builders.ts"
import { fractalBoard, fractalPanel } from "../../components/fractal.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send, Shade } from "../../types.ts"

const PRESETS = [
  "-0.4+0.6i",
  "-0.8+0.156i",
  "0.285+0.01i",
  "-0.727+0.189i",
  "-0.1+0.651i",
  "0.355+0.355i",
  "custom",
]

type State = {
  steps: number
  settings: {
    preset: string
    cre: number
    cim: number
    zoom: number
    cycle: number
    band: number
    drift: number
    fade: number
    spin: number
    depth: number
    primary: string
    accent: string
  }
  frame: { width: number; height: number; rows: number[][]; palette: string[] }
  shade?: Shade
}

const turn = setter("julia")

export function julia(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="julia">
      <card key="board">
        {fractalBoard("julia", "julia", s.frame, { steps: s.steps, shade: s.shade })}
      </card>
      <card key="seed">
        <choice key="preset" value={s.settings.preset} options={PRESETS} call={turn("preset")} arg="value" label="preset" />
        <range key="cre" value={s.settings.cre} min={-2} max={2} step={0.01} call={turn("cre")} arg="value" label="cre" />
        <range key="cim" value={s.settings.cim} min={-2} max={2} step={0.01} call={turn("cim")} arg="value" label="cim" />
      </card>
      {fractalPanel(turn, s.settings)}
    </stack>
  )
}
