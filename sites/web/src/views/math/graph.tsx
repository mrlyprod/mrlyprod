import {
  Box,
  Button,
  Caption,
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

const KINDS = ["core", "tunnel", "edge"]

type State = {
  design: string
  index: number
  count: number
  number: number
  level: number
  kind: string
  fill: string
  alpha: number
  particles: boolean
  segments: boolean
  ghost: boolean
  axes: boolean
  census: {
    nodes: number
    branches: number
    tips: number
    junctions: number
    components: number
    length: number
    dimension: number
  }
  shade?: Shade
}

export function Graph({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("graph")
  const { rig, turn: spin, pan, zoom, onOrtho } = useOrbit("graph")
  return (
    <Stack>
      <Box>
        <Shader shade={s.shade} handle="graph" turn={spin} pan={pan} zoom={zoom} />
      </Box>
      <Box>
        <Pager
          current={s.index + 1}
          total={s.count}
          onPrev={() => send(call("graph.page", { dir: "prev" }))}
          onNext={() => send(call("graph.page", { dir: "next" }))}
        />
        <Cluster>
          <Button onClick={() => send(call("face.full", { handle: "graph" }))}>fullscreen</Button>
          <Button onClick={() => send(call("graph.reset"))}>reset</Button>
          <Shot />
        </Cluster>
      </Box>
      <Section label={s.design}>
        <Cluster>
          <Chip>{`nodes ${s.census.nodes}`}</Chip>
          <Chip>{`branches ${s.census.branches}`}</Chip>
          <Chip>{`tips ${s.census.tips}`}</Chip>
          <Chip>{`junctions ${s.census.junctions}`}</Chip>
          <Chip>{`components ${s.census.components}`}</Chip>
          <Chip>{`length ${(s.census.length / 1000).toFixed(3)}`}</Chip>
          <Chip>{`dimension ${(s.census.dimension / 1e15).toFixed(3)}`}</Chip>
        </Cluster>
        <Caption>tips orange, through gray, junctions purple, alone cyan</Caption>
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
        <Choice
          label="kind"
          mode="row"
          value={s.kind}
          options={opts(KINDS)}
          onChange={v => send(turn("kind", v))}
        />
      </Section>
      <Section label="paint">
        <Choice label="fill" value={s.fill} options={opts(colors())} onChange={v => send(turn("fill", v))} />
        <Field label="alpha">
          <Slider min={32} max={255} step={1} value={s.alpha} onChange={v => send(turn("alpha", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Field label="particles">
          <Toggle value={s.particles} onChange={v => send(turn("particles", v))} />
        </Field>
        <Field label="segments">
          <Toggle value={s.segments} onChange={v => send(turn("segments", v))} />
        </Field>
        <Field label="ghost">
          <Toggle value={s.ghost} onChange={v => send(turn("ghost", v))} />
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
