import { setter } from "../../builders.ts"
import { fractalBoard } from "../../components/fractal.tsx"
import { palette } from "../../components/palette.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  steps: number
  settings: { cols: number; rows: number; size: number; speed: number; scale: number; palette: string[] }
  frame: { width: number; height: number; rows: number[][]; palette: string[] }
}

const turn = setter("sleep")

export function sleep(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="sleep">
      <card key="board">
        {fractalBoard("sleep", "sleep", s.frame, { steps: s.steps })}
      </card>
      <card key="grid">
        <range key="cols" value={s.settings.cols} min={8} max={64} step={1} call={turn("cols")} arg="value" label="cols" />
        <range key="rows" value={s.settings.rows} min={8} max={48} step={1} call={turn("rows")} arg="value" label="rows" />
        <range key="size" value={s.settings.size} min={2} max={12} step={1} call={turn("size")} arg="value" label="size" />
        <range key="scale" value={s.settings.scale} min={2} max={16} step={1} call={turn("scale")} arg="value" label="scale" />
      </card>
      <card key="motion">
        <range key="speed" value={s.settings.speed} min={0.1} max={2.0} step={0.1} call={turn("speed")} arg="value" label="speed" />
      </card>
      <card key="paint">
        {palette("sleep", s.settings.palette)}
      </card>
    </stack>
  )
}
