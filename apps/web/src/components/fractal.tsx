import {
  Box,
  Button,
  Caption,
  Cluster,
  ColorPicker,
  Field,
  Section,
  Slider,
} from "mrlyui"
import type { Point, Swatch } from "mrlyui"
import { call, setter } from "../builders"
import { Shader } from "../eyes/Shader"
import { hex, names } from "../palette"
import { useSend } from "../send"
import { Shot } from "./Shot"
import type { Shade } from "../types"

export type Dials = {
  width: number
  height: number
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

// SWATCHES

function pool(): Swatch<string>[] {
  return names().map(name => ({ name, color: hex(name) }))
}

function named(value: string): string | null {
  return names().find(name => hex(name).toLowerCase() === value.toLowerCase()) ?? null
}

// FRACTAL

export function Fractal({ app, shade, grid, steps }: {
  app: string
  shade: Shade | undefined
  grid: Point
  steps: number
}) {
  const send = useSend()
  return (
    <Box>
      <Shader shade={shade} grid={grid} handle={app} />
      <Caption>{`steps ${steps}`}</Caption>
      <Cluster>
        <Button onClick={() => send(call("face.full", { handle: app }))}>fullscreen</Button>
        <Button onClick={() => send(call(`${app}.reset`))}>reset</Button>
        <Shot />
      </Cluster>
    </Box>
  )
}

// PANEL

export function Panel({ app, dials }: { app: string; dials: Dials }) {
  const send = useSend()
  const turn = setter(app)
  const ink = (key: "primary" | "accent") => (name: string | null) => {
    if (name !== null) send(turn(key, hex(name)))
  }
  return (
    <>
      <Section label="motion">
        <Field label="zoom">
          <Slider
            min={1}
            max={1.05}
            step={0.001}
            value={dials.zoom / 1000}
            format={v => v.toFixed(3)}
            onChange={v => send(turn("zoom", Math.round(v * 1000)))}
          />
        </Field>
        <Field label="cycle">
          <Slider min={30} max={3000} step={30} value={dials.cycle} onChange={v => send(turn("cycle", v))} />
        </Field>
        <Field label="drift">
          <Slider
            min={0}
            max={4}
            step={0.1}
            value={dials.drift / 1000}
            format={v => v.toFixed(1)}
            onChange={v => send(turn("drift", Math.round(v * 1000)))}
          />
        </Field>
        <Field label="spin">
          <Slider
            min={0}
            max={0.05}
            step={0.001}
            value={dials.spin / 1000}
            format={v => v.toFixed(3)}
            onChange={v => send(turn("spin", Math.round(v * 1000)))}
          />
        </Field>
        <Field label="fade">
          <Slider min={0} max={240} step={8} value={dials.fade} onChange={v => send(turn("fade", v))} />
        </Field>
      </Section>
      <Section label="paint">
        <Field label="band">
          <Slider
            min={2}
            max={64}
            step={1}
            value={dials.band / 1000}
            onChange={v => send(turn("band", Math.round(v * 1000)))}
          />
        </Field>
        <Field label="depth">
          <Slider min={16} max={600} step={8} value={dials.depth} onChange={v => send(turn("depth", v))} />
        </Field>
        <Field label="primary">
          <ColorPicker swatches={pool()} value={named(dials.primary)} onChange={ink("primary")} />
        </Field>
        <Field label="accent">
          <ColorPicker swatches={pool()} value={named(dials.accent)} onChange={ink("accent")} />
        </Field>
      </Section>
      <Section label="frame">
        <Field label="width">
          <Slider min={16} max={512} step={16} value={dials.width} onChange={v => send(turn("width", v))} />
        </Field>
        <Field label="height">
          <Slider min={16} max={512} step={16} value={dials.height} onChange={v => send(turn("height", v))} />
        </Field>
      </Section>
    </>
  )
}
