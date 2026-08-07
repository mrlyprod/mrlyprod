import {
  Box,
  Button,
  Caption,
  Choice,
  Field,
  Grid,
  Section,
  Slider,
  Stack,
} from "mrlyui"
import { call, set } from "../../builders"
import { GameOver } from "../../components/GameOver"
import { DESIGNS_SOLID as DESIGNS, opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

type State = {
  score: number
  steps: number
  over: boolean
  settings: {
    cols: number
    rows: number
    kinds: number
    speed: number
    design: string
  }
  cells: Deck
}

export function Crush({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  return (
    <Stack>
      <Box>
        <Caption>{`💎 ${s.score} · steps ${s.steps}`}</Caption>
        <Cells app="crush" cells={s.cells} />
      </Box>
      {s.over ? <GameOver app="crush" emoji="🍬" status={`score ${s.score}`} /> : null}
      <Box>
        {s.over ? null : (
          <Grid cols={4}>
            <Button onClick={() => send(call("crush.move", { dir: "left" }))}>←</Button>
            <Button onClick={() => send(call("crush.crush"))}>crush</Button>
            <Button onClick={() => send(call("crush.drop"))}>drop</Button>
            <Button onClick={() => send(call("crush.move", { dir: "right" }))}>→</Button>
          </Grid>
        )}
        <Shot />
      </Box>
      <Section label="rules">
        <Field label="cols">
          <Slider min={4} max={16} step={1} value={s.settings.cols} onChange={v => send(set("crush", "cols", v))} />
        </Field>
        <Field label="rows">
          <Slider min={4} max={16} step={1} value={s.settings.rows} onChange={v => send(set("crush", "rows", v))} />
        </Field>
        <Field label="kinds">
          <Slider min={2} max={8} step={1} value={s.settings.kinds} onChange={v => send(set("crush", "kinds", v))} />
        </Field>
      </Section>
      <Section label="speed">
        <Field label="speed">
          <Slider min={1} max={8} step={1} value={s.settings.speed} onChange={v => send(set("crush", "speed", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Choice
          label="design"
          mode="cycle"
          options={opts(DESIGNS)}
          value={s.settings.design}
          onChange={v => send(set("crush", "design", v))}
        />
      </Section>
    </Stack>
  )
}
