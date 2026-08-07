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

export function shadeBoard(handle: string, shade: Shade | undefined, grid: [number, number], steps: number): Node[] {
  return [
    <canvas key="frame" handle={handle} shade={shade} grid={grid} />,
    <button key="full" call={call("face.full", { handle })}>fullscreen</button>,
    <text key="meter" role="note">{`steps ${steps}`}</text>,
    <Shot />,
  ]
}

export function fractalPanel(turn: (key: string) => Call, settings: Dials): Node[] {
  return [
    <card key="motion">
      <range key="zoom" value={settings.zoom} min={1000} max={1050} step={1} scale={1000} call={turn("zoom")} arg="value" label="zoom" />
      <range key="cycle" value={settings.cycle} min={30} max={3000} step={30} call={turn("cycle")} arg="value" label="cycle" />
      <range key="drift" value={settings.drift} min={0} max={4000} step={100} scale={1000} call={turn("drift")} arg="value" label="drift" />
      <range key="spin" value={settings.spin} min={0} max={50} step={1} scale={1000} call={turn("spin")} arg="value" label="spin" />
    </card>,
    <card key="paint">
      <range key="band" value={settings.band} min={2000} max={64000} step={1000} scale={1000} call={turn("band")} arg="value" label="band" />
      <range key="fade" value={settings.fade} min={0} max={240} step={8} call={turn("fade")} arg="value" label="fade" />
      <range key="depth" value={settings.depth} min={16} max={600} step={8} call={turn("depth")} arg="value" label="depth" />
      <field key="primary" value={settings.primary} live={false} call={turn("primary")} arg="value" label="primary" />
      <field key="accent" value={settings.accent} live={false} call={turn("accent")} arg="value" label="accent" />
    </card>,
  ]
}
