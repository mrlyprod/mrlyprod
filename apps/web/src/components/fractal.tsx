import { call } from "../builders.ts"
import { h } from "../jsx.ts"
import { Shot } from "./Shot.tsx"
import type { Call, Node, Shade } from "../types.ts"

type Dials = {
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

export function fractalBoard(app: string, handle: string, frame: { width: number; height: number; rows: number[][]; palette: string[] }, opts: { steps: number; shade?: Shade }): Node[] {
  return [
    <canvas key="frame" handle={handle} rows={frame.rows} palette={frame.palette} shade={opts.shade} grid={[frame.width, frame.height]} />,
    <button key="full" call={call("face.full", { handle })}>fullscreen</button>,
    <text key="meter" role="note">{`steps ${opts.steps}`}</text>,
    <Shot />,
  ]
}

export function fractalPanel(turn: (key: string) => Call, settings: Dials): Node[] {
  return [
    <card key="motion">
      <range key="zoom" value={settings.zoom} min={1.0} max={1.05} step={0.001} call={turn("zoom")} arg="value" label="zoom" />
      <range key="cycle" value={settings.cycle} min={30} max={3000} step={30} call={turn("cycle")} arg="value" label="cycle" />
      <range key="drift" value={settings.drift} min={0} max={4} step={0.1} call={turn("drift")} arg="value" label="drift" />
      <range key="spin" value={settings.spin} min={0} max={0.05} step={0.001} call={turn("spin")} arg="value" label="spin" />
    </card>,
    <card key="paint">
      <range key="band" value={settings.band} min={2} max={64} step={1} call={turn("band")} arg="value" label="band" />
      <range key="fade" value={settings.fade} min={0} max={240} step={8} call={turn("fade")} arg="value" label="fade" />
      <range key="depth" value={settings.depth} min={16} max={600} step={8} call={turn("depth")} arg="value" label="depth" />
      <field key="primary" value={settings.primary} live={false} call={turn("primary")} arg="value" label="primary" />
      <field key="accent" value={settings.accent} live={false} call={turn("accent")} arg="value" label="accent" />
    </card>,
  ]
}
