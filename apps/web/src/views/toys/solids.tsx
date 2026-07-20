import { call, setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { Shot } from "../../components/Shot.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send, Shade } from "../../types.ts"

type State = {
  object: string
  spin: number
  camera: { yaw: number; pitch: number; dist: number; pan: [number, number]; ortho: boolean }
  settings: {
    size: number
    bands: number
    speed: number
    light_yaw: number
    light_pitch: number
    alpha: number
    edges: boolean
    wireframe: boolean
    axes: boolean
  }
  frame: { rows: number[][]; palette: string[] }
  shade?: Shade
}

const turn = setter("solids")

export function solids(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="solids">
      <card key="board">
        <Board
          app="solids"
          keyName="solid"
          rows={s.frame.rows}
          palette={s.frame.palette}
          shade={s.shade}
          turn={call("solids.turn")}
          zoom={call("solids.zoom")}
          pan={call("solids.pan")}
        />
      </card>
      <card key="solids">
        <button key="cube" call={call("solids.pick", { solid: "cube" })}>cube</button>
        <button key="tetra" call={call("solids.pick", { solid: "tetra" })}>tetra</button>
        <button key="octa" call={call("solids.pick", { solid: "octa" })}>octa</button>
        <button key="icosa" call={call("solids.pick", { solid: "icosa" })}>icosa</button>
        <Shot />
      </card>
      <card key="meter">
        <text key="meter" role="note">{`${s.object} · spin ${s.spin}`}</text>
      </card>
      <card key="looks">
        <toggle key="edges" on={s.settings.edges} call={turn("edges")} arg="value" label="edges" />
        <toggle key="wireframe" on={s.settings.wireframe} call={turn("wireframe")} arg="value" label="wireframe" />
        <toggle key="axes" on={s.settings.axes} call={turn("axes")} arg="value" label="axes" />
        <toggle key="ortho" on={s.camera.ortho} call={turn("ortho")} arg="value" label="ortho" />
        <range key="alpha" value={s.settings.alpha} min={32} max={255} step={1} call={turn("alpha")} arg="value" label="alpha" />
      </card>
      <card key="settings">
        <range key="size" value={s.settings.size} min={32} max={160} step={1} call={turn("size")} arg="value" label="size" />
        <range key="bands" value={s.settings.bands} min={2} max={8} step={1} call={turn("bands")} arg="value" label="bands" />
        <range key="speed" value={s.settings.speed} min={0} max={16} step={1} call={turn("speed")} arg="value" label="speed" />
        <range key="light-yaw" value={s.settings.light_yaw} min={0} max={255} step={1} call={turn("light_yaw")} arg="value" label="light yaw" />
        <range key="light-pitch" value={s.settings.light_pitch} min={-56} max={56} step={1} call={turn("light_pitch")} arg="value" label="light pitch" />
      </card>
    </stack>
  )
}
