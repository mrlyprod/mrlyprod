import { call, setter } from "../../builders.ts"
import { Shot } from "../../components/Shot.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send, Shade } from "../../types.ts"

const ANGLES = ["0", "90", "180", "270"]

const LATTICES = ["square", "hex"]

type State = {
  offset: number
  angle: number
  lattice: string
  shade?: Shade
}

const turn = setter("moire")

export function moire(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="moire">
      <card key="board">
        <canvas key="frame" handle="moire" shade={s.shade} grid={[1, 1]} />
        <button key="full" call={call("face.full", { handle: "moire" })}>fullscreen</button>
      </card>
      <card key="controls">
        <range key="offset" value={s.offset} min={-6} max={6} step={1} call={turn("offset")} arg="value" label="offset" />
        <choice key="angle" value={String(s.angle)} options={ANGLES} call={turn("angle")} arg="value" label="angle" mode="row" />
        <choice key="lattice" value={s.lattice} options={LATTICES} call={turn("lattice")} arg="value" label="lattice" mode="row" />
        <button key="reset" call={call("moire.reset")}>reset</button>
        <Shot />
      </card>
    </stack>
  )
}
