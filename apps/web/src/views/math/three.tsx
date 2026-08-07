import {
  Box,
  Button,
  Chip,
  Choice,
  Cluster,
  Field,
  Pager,
  Section,
  Slider,
  Stack,
  Toggle,
} from "mrlyui"
import { call, setter } from "../../builders"
import { colors, NUMBERS, opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { useOrbit } from "../../eyes/orbit"
import { Shader } from "../../eyes/Shader"
import { useSend } from "../../send"
import type { Shade } from "../../types"

const LEVELS = ["1", "2", "3"]

type State = {
  design: string
  index: number
  count: number
  number: number
  level: number
  fill: string
  alpha: number
  edges: boolean
  wireframe: boolean
  axes: boolean
  census: { grid: number; fill: number; void: number }
  shade?: Shade
}

export function Three({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("three")
  const { rig, turn: spin, pan, zoom, onOrtho } = useOrbit("three")
  return (
    <Stack>
      <Box>
        <Shader shade={s.shade} handle="three" turn={spin} pan={pan} zoom={zoom} />
      </Box>
      <Box>
        <Pager
          current={s.index + 1}
          total={s.count}
          onPrev={() => send(call("three.page", { dir: "prev" }))}
          onNext={() => send(call("three.page", { dir: "next" }))}
        />
        <Cluster>
          <Button onClick={() => send(call("face.full", { handle: "three" }))}>fullscreen</Button>
          <Button onClick={() => send(call("three.reset"))}>reset</Button>
          <Shot />
        </Cluster>
      </Box>
      <Section label={s.design}>
        <Cluster>
          <Chip>{`grid ${s.census.grid}^3`}</Chip>
          <Chip>{`fill ${s.census.fill}`}</Chip>
          <Chip>{`void ${s.census.void}`}</Chip>
        </Cluster>
      </Section>
      <Section label="shape">
        <Choice
          label="number"
          mode="row"
          value={String(s.number)}
          options={opts(NUMBERS)}
          onChange={v => send(turn("number", v))}
        />
        <Choice
          label="level"
          mode="row"
          value={String(s.level)}
          options={opts(LEVELS)}
          onChange={v => send(turn("level", v))}
        />
      </Section>
      <Section label="paint">
        <Choice label="fill" value={s.fill} options={opts(colors())} onChange={v => send(turn("fill", v))} />
        <Field label="alpha">
          <Slider min={32} max={255} step={1} value={s.alpha} onChange={v => send(turn("alpha", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Field label="edges">
          <Toggle value={s.edges} onChange={v => send(turn("edges", v))} />
        </Field>
        <Field label="wireframe">
          <Toggle value={s.wireframe} onChange={v => send(turn("wireframe", v))} />
        </Field>
        <Field label="axes">
          <Toggle value={s.axes} onChange={v => send(turn("axes", v))} />
        </Field>
        <Field label="ortho">
          <Toggle value={rig.ortho} onChange={onOrtho} />
        </Field>
      </Section>
    </Stack>
  )
}
