import {
  Box,
  Button,
  Chip,
  Cluster,
  Field,
  Section,
  Slider,
  Stack,
  Toggle,
} from "mrlyui"
import { call, setter } from "../../builders"
import { Shot } from "../../components/Shot"
import { useOrbit } from "../../eyes/orbit"
import { Shader } from "../../eyes/Shader"
import { useSend } from "../../send"
import type { Shade } from "../../types"

const SOLIDS = ["cube", "tetra", "octa", "icosa"]

type State = {
  object: string
  spin: number
  settings: {
    bands: number
    speed: number
    light_yaw: number
    light_pitch: number
    alpha: number
    edges: boolean
    wireframe: boolean
    axes: boolean
  }
  shade?: Shade
}

export function Solids({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("solids")
  const { rig, turn: orbit, pan, zoom, onOrtho } = useOrbit("solids")
  return (
    <Stack>
      <Box>
        <Shader shade={s.shade} handle="solids" turn={orbit} pan={pan} zoom={zoom} />
        <Cluster>
          <Button onClick={() => send(call("face.full", { handle: "solids" }))}>fullscreen</Button>
          <Button onClick={() => send(call("solids.reset"))}>reset</Button>
          <Shot />
        </Cluster>
      </Box>
      <Section label="solid">
        <Cluster>
          {SOLIDS.map(solid => (
            <Button key={solid} active={s.object === solid} onClick={() => send(call("solids.pick", { solid }))}>
              {solid}
            </Button>
          ))}
        </Cluster>
        <Cluster>
          <Chip>{`spin ${s.spin}`}</Chip>
        </Cluster>
        <Field label="speed">
          <Slider min={0} max={16} step={1} value={s.settings.speed} onChange={v => send(turn("speed", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Field label="edges">
          <Toggle value={s.settings.edges} onChange={v => send(turn("edges", v))} />
        </Field>
        <Field label="wireframe">
          <Toggle value={s.settings.wireframe} onChange={v => send(turn("wireframe", v))} />
        </Field>
        <Field label="axes">
          <Toggle value={s.settings.axes} onChange={v => send(turn("axes", v))} />
        </Field>
        <Field label="ortho">
          <Toggle value={rig.ortho} onChange={onOrtho} />
        </Field>
        <Field label="alpha">
          <Slider min={32} max={255} step={1} value={s.settings.alpha} onChange={v => send(turn("alpha", v))} />
        </Field>
      </Section>
      <Section label="light">
        <Field label="bands">
          <Slider min={2} max={8} step={1} value={s.settings.bands} onChange={v => send(turn("bands", v))} />
        </Field>
        <Field label="yaw">
          <Slider
            min={0}
            max={255}
            step={1}
            value={s.settings.light_yaw}
            onChange={v => send(turn("light_yaw", v))}
          />
        </Field>
        <Field label="pitch">
          <Slider
            min={-56}
            max={56}
            step={1}
            value={s.settings.light_pitch}
            onChange={v => send(turn("light_pitch", v))}
          />
        </Field>
      </Section>
    </Stack>
  )
}
