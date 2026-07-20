import { setter } from "../../builders.ts"
import { fractalBoard } from "../../components/fractal.tsx"
import { palette } from "../../components/palette.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  steps: number
  settings: { cols: number; rows: number; speed: number; trail: number; rainbow: boolean; charset: string; palette: string[] }
  frame: { width: number; height: number; rows: number[][]; palette: string[] }
}

const turn = setter("matrix")

export function matrix(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="matrix">
      <card key="board">
        {fractalBoard("matrix", "matrix", s.frame, { steps: s.steps })}
      </card>
      <card key="grid">
        <range key="cols" value={s.settings.cols} min={4} max={128} step={1} call={turn("cols")} arg="value" label="cols" />
        <range key="rows" value={s.settings.rows} min={4} max={96} step={1} call={turn("rows")} arg="value" label="rows" />
      </card>
      <card key="rain">
        <range key="speed" value={s.settings.speed} min={1} max={4} step={1} call={turn("speed")} arg="value" label="speed" />
        <range key="trail" value={s.settings.trail} min={2} max={32} step={1} call={turn("trail")} arg="value" label="trail" />
        <toggle key="rainbow" on={s.settings.rainbow} call={turn("rainbow")} arg="value" label="rainbow" />
      </card>
      <card key="paint">
        <field key="charset" value={s.settings.charset} live={false} call={turn("charset")} arg="value" label="charset" />
        {palette("matrix", "matrix", s.settings.palette)}
      </card>
    </stack>
  )
}
