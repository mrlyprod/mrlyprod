import { setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { palette } from "../../components/palette.tsx"
import { Shot } from "../../components/Shot.tsx"
import { h } from "../../jsx.ts"
import type { Cells, Node, Send } from "../../types.ts"

type State = {
  steps: number
  play: boolean
  settings: { cols: number; rows: number; speed: number; trail: number; palette: string[] }
  cells: Cells
}

const turn = setter("matrix")

export function matrix(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="matrix">
      <card key="board">
        <Board app="matrix" cells={s.cells} />
        <toggle key="play" on={s.play} call={turn("play")} arg="value" label="play" />
        <text key="meter" role="note">{`steps ${s.steps}`}</text>
        <Shot />
      </card>
      <card key="grid">
        <range key="cols" value={s.settings.cols} min={4} max={64} step={1} call={turn("cols")} arg="value" label="cols" />
        <range key="rows" value={s.settings.rows} min={4} max={64} step={1} call={turn("rows")} arg="value" label="rows" />
      </card>
      <card key="rain">
        <range key="speed" value={s.settings.speed} min={1} max={4} step={1} call={turn("speed")} arg="value" label="speed" />
        <range key="trail" value={s.settings.trail} min={2} max={32} step={1} call={turn("trail")} arg="value" label="trail" />
      </card>
      <card key="paint">{palette("matrix", s.settings.palette)}</card>
    </stack>
  )
}
