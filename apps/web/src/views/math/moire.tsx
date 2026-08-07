import {
  Box,
  Button,
  Choice,
  Cluster,
  Field,
  Section,
  Slider,
  Stack,
} from "mrlyui"
import { call, setter } from "../../builders"
import { opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Shader } from "../../eyes/Shader"
import { useSend } from "../../send"
import type { Shade } from "../../types"

const ANGLES = ["0", "90", "180", "270"]

const LATTICES = ["square", "hex"]

type State = {
  offset: number
  angle: number
  lattice: string
  shade?: Shade
}

export function Moire({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("moire")
  return (
    <Stack>
      <Box>
        <Shader shade={s.shade} grid={[1, 1]} handle="moire" />
        <Cluster>
          <Button onClick={() => send(call("face.full", { handle: "moire" }))}>fullscreen</Button>
          <Button onClick={() => send(call("moire.reset"))}>reset</Button>
          <Shot />
        </Cluster>
      </Box>
      <Section label="overlay">
        <Field label="offset">
          <Slider min={-6} max={6} step={1} value={s.offset} onChange={v => send(turn("offset", v))} />
        </Field>
        <Choice
          label="angle"
          mode="row"
          value={String(s.angle)}
          options={opts(ANGLES)}
          onChange={v => send(turn("angle", v))}
        />
      </Section>
      <Section label="lattice">
        <Choice
          mode="row"
          value={s.lattice}
          options={opts(LATTICES)}
          onChange={v => send(turn("lattice", v))}
        />
      </Section>
    </Stack>
  )
}
