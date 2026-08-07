import { call, setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { Shot } from "../../components/Shot.tsx"
import { h } from "../../jsx.ts"
import type { Cells, Node, Send } from "../../types.ts"

type State = {
  steps: number
  painted: number
  settings: { width: number; height: number }
  cells: Cells
}

const turn = setter("pixel")

export function pixel(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="pixel">
      <card key="board">
        <Board
          app="pixel"
          cells={s.cells}
          drag={call("pixel.stroke")}
          grid={[s.settings.width, s.settings.height]}
        />
      </card>
      <card key="controls">
        <button key="clear" call={call("pixel.clear")}>clear</button>
        <Shot />
      </card>
      <card key="meter">
        <text key="meter" role="note">{`painted ${s.painted} · strokes ${s.steps}`}</text>
      </card>
      <card key="settings">
        <range key="width" value={s.settings.width} min={4} max={64} step={1} call={turn("width")} arg="value" label="width" />
        <range key="height" value={s.settings.height} min={4} max={64} step={1} call={turn("height")} arg="value" label="height" />
      </card>
    </stack>
  )
}
