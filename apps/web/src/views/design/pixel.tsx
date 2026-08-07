import {
  Box,
  Button,
  Caption,
  Cluster,
  Field,
  Section,
  Slider,
  Stack,
} from "mrlyui"
import { call, set } from "../../builders"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

type State = {
  steps: number
  painted: number
  settings: { width: number; height: number }
  cells: Deck
}

export function Pixel({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  return (
    <Stack>
      <Box>
        <Cells
          app="pixel"
          cells={s.cells}
          grid={[s.settings.width, s.settings.height]}
          handle="pixel"
          onDrag={points => send(call("pixel.stroke", { points }))}
        />
      </Box>
      <Box>
        <Cluster>
          <Button onClick={() => send(call("pixel.clear"))}>clear</Button>
          <Button onClick={() => send(call("pixel.reset"))}>new ink</Button>
          <Shot />
        </Cluster>
        <Caption>{`painted ${s.painted} · strokes ${s.steps}`}</Caption>
      </Box>
      <Section label="canvas">
        <Field label="width">
          <Slider
            min={4}
            max={64}
            step={1}
            value={s.settings.width}
            onChange={v => send(set("pixel", "width", v))}
          />
        </Field>
        <Field label="height">
          <Slider
            min={4}
            max={64}
            step={1}
            value={s.settings.height}
            onChange={v => send(set("pixel", "height", v))}
          />
        </Field>
      </Section>
    </Stack>
  )
}
