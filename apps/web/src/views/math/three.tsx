import { call, setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Pager } from "../../components/Pager.tsx"
import { colors, NUMBERS } from "../../components/options.ts"
import { h } from "../../jsx.ts"
import type { Node, Send, Shade } from "../../types.ts"

const VIEWS = ["iso", "front", "top"]

type State = {
  design: string
  index: number
  count: number
  number: number
  level: number
  view: string
  fill: string
  alpha: number
  edges: boolean
  wireframe: boolean
  axes: boolean
  camera: { yaw: number; pitch: number; dist: number; pan: [number, number]; ortho: boolean }
  census: { grid: number; fill: number; void: number }
  frame: { rows: number[][]; palette: string[] }
  shade?: Shade
}

const turn = setter("three")

export function three(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="three">
      <card key="board">
        <Board
          app="three"
          rows={s.frame.rows}
          palette={s.frame.palette}
          shade={s.shade}
          turn={call("three.turn")}
          zoom={call("three.zoom")}
          pan={call("three.pan")}
        />
      </card>
      <card key="page">
        <Pager app="three" current={s.index + 1} total={s.count} />
        <button key="reset" call={call("three.reset")}>reset</button>
        <Shot />
      </card>
      <card key="controls">
        <choice key="number" value={String(s.number)} options={NUMBERS} call={turn("number")} arg="value" label="number" mode="row" />
        <choice key="level" value={String(s.level)} options={["1", "2", "3"]} call={turn("level")} arg="value" label="level" mode="row" />
        <choice key="view" value={s.view} options={VIEWS} call={turn("view")} arg="value" label="view" mode="row" />
        <choice key="fill" value={s.fill} options={colors()} call={turn("fill")} arg="value" label="fill" />
      </card>
      <card key="looks">
        <toggle key="edges" on={s.edges} call={turn("edges")} arg="value" label="edges" />
        <toggle key="wireframe" on={s.wireframe} call={turn("wireframe")} arg="value" label="wireframe" />
        <toggle key="axes" on={s.axes} call={turn("axes")} arg="value" label="axes" />
        <toggle key="ortho" on={s.camera.ortho} call={turn("ortho")} arg="value" label="ortho" />
        <range key="alpha" value={s.alpha} min={32} max={255} step={1} call={turn("alpha")} arg="value" label="alpha" />
      </card>
      <card key="meter">
        <text key="meter" role="note">{`${s.design} · ${s.census.grid}^3 · fill ${s.census.fill} · void ${s.census.void}`}</text>
      </card>
    </stack>
  )
}
