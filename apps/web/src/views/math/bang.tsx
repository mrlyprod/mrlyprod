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
  Stack,
  Toggle,
} from "mrlyui"
import { call, setter } from "../../builders"
import { opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { useOrbit } from "../../eyes/orbit"
import { Shader } from "../../eyes/Shader"
import { useSend } from "../../send"
import type { Cells as Deck, Shade } from "../../types"

const DIMENSIONS = ["1", "2", "3"]

type State = {
  dimension: number
  base: number
  index: number
  count: number
  name: string
  code: string
  degree: number
  anf: string
  cells?: Deck
  shade?: Shade
}

export function Bang({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("bang")
  const { rig, turn: spin, pan, zoom, onOrtho } = useOrbit("bang")
  const solid = s.dimension === 3
  return (
    <Stack>
      <Box>
        {solid || s.cells === undefined ? (
          <Shader shade={s.shade} handle="bang" turn={spin} pan={pan} zoom={zoom} />
        ) : (
          <Cells app="bang" cells={s.cells} handle="bang" />
        )}
      </Box>
      <Box>
        <Pager
          current={s.index + 1}
          total={s.count}
          onPrev={() => send(call("bang.page", { dir: "prev" }))}
          onNext={() => send(call("bang.page", { dir: "next" }))}
        />
        <Cluster>
          <Button onClick={() => send(call("face.full", { handle: "bang" }))}>fullscreen</Button>
          <Button onClick={() => send(call("bang.reset"))}>reset</Button>
          <Shot />
        </Cluster>
      </Box>
      <Section label="walk">
        <Choice
          label="dimension"
          mode="row"
          value={String(s.dimension)}
          options={opts(DIMENSIONS)}
          onChange={v => send(turn("dimension", v))}
        />
        {solid && (
          <Field label="ortho">
            <Toggle value={rig.ortho} onChange={onOrtho} />
          </Field>
        )}
      </Section>
      <Section label={s.name}>
        <Cluster>
          <Chip>{`base ${s.base}`}</Chip>
          <Chip>{`code ${s.code}`}</Chip>
          <Chip>{`degree ${s.degree}`}</Chip>
        </Cluster>
        <Caption>{s.anf}</Caption>
      </Section>
    </Stack>
  )
}
