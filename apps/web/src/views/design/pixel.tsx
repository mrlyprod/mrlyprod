import { call, setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { DESIGNS_SOLID as DESIGNS } from "../../components/options.ts"
import { Shot } from "../../components/Shot.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  steps: number
  painted: number
  settings: { width: number; height: number; design: string }
  frame: { rows: number[][]; palette: string[] }
}

const turn = setter("pixel")

export function pixel(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="pixel">
      <card key="board">
        <Board
          app="pixel"
          rows={s.frame.rows}
          palette={s.frame.palette}
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
        <choice key="design" value={s.settings.design} options={DESIGNS} call={turn("design")} arg="value" label="design" mode="row" />
        <range key="width" value={s.settings.width} min={4} max={64} step={1} call={turn("width")} arg="value" label="width" />
        <range key="height" value={s.settings.height} min={4} max={64} step={1} call={turn("height")} arg="value" label="height" />
      </card>
    </stack>
  )
}
