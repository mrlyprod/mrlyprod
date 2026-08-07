import {
  Box,
  Button,
  Caption,
  Cluster,
  Field,
  Section,
  Slider,
  Stack,
  Symbol,
} from "mrlyui"
import { call, setter } from "../../builders"
import { Palette } from "../../components/Palette"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

type State = {
  steps: number
  play: boolean
  settings: { cols: number; rows: number; speed: number; trail: number; palette: string[] }
  cells: Deck
}

export function Matrix({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const turn = setter("matrix")
  return (
    <Stack>
      <Box>
        <Cells app="matrix" cells={s.cells} handle="matrix" />
        <Caption>{`steps ${s.steps}`}</Caption>
        <Cluster>
          <Button active={s.play} onClick={() => send(turn("play", !s.play))}>
            <Symbol name={s.play ? "pause" : "play_arrow"} />
          </Button>
          <Button onClick={() => send(call("face.full", { handle: "matrix" }))}>fullscreen</Button>
          <Button onClick={() => send(call("matrix.reset"))}>reset</Button>
          <Shot />
        </Cluster>
      </Box>
      <Section label="field">
        <Field label="cols">
          <Slider min={4} max={64} step={1} value={s.settings.cols} onChange={v => send(turn("cols", v))} />
        </Field>
        <Field label="rows">
          <Slider min={4} max={64} step={1} value={s.settings.rows} onChange={v => send(turn("rows", v))} />
        </Field>
      </Section>
      <Section label="rain">
        <Field label="speed">
          <Slider min={1} max={4} step={1} value={s.settings.speed} onChange={v => send(turn("speed", v))} />
        </Field>
        <Field label="trail">
          <Slider min={2} max={32} step={1} value={s.settings.trail} onChange={v => send(turn("trail", v))} />
        </Field>
      </Section>
      <Section label="palette">
        <Palette app="matrix" colors={s.settings.palette} />
      </Section>
    </Stack>
  )
}
