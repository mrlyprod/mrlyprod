import {
  Box,
  Caption,
  Choice,
  Field,
  Section,
  Slider,
  Stack,
  Toggle,
} from "mrlyui"
import { set } from "../../builders"
import { DPad } from "../../components/DPad"
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
    grid: number
    apples: number
    wrap: boolean
    self_collision: boolean
    speed: number
    design: string
  }
  cells: Deck
}

export function Snake({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  return (
    <Stack>
      <Box>
        <Caption>{`🍎 ${s.score} · steps ${s.steps}`}</Caption>
        <Cells app="snake" cells={s.cells} />
      </Box>
      {s.over ? <GameOver app="snake" emoji="🐍" status={`score ${s.score}`} /> : null}
      <Box>
        {s.over ? null : <DPad app="snake" verb="turn" />}
        <Shot />
      </Box>
      <Section label="rules">
        <Field label="grid">
          <Slider min={5} max={64} step={1} value={s.settings.grid} onChange={v => send(set("snake", "grid", v))} />
        </Field>
        <Field label="apples">
          <Slider min={1} max={16} step={1} value={s.settings.apples} onChange={v => send(set("snake", "apples", v))} />
        </Field>
        <Field label="wrap">
          <Toggle value={s.settings.wrap} onChange={v => send(set("snake", "wrap", v))} />
        </Field>
        <Field label="self collision">
          <Toggle
            value={s.settings.self_collision}
            onChange={v => send(set("snake", "self_collision", v))}
          />
        </Field>
      </Section>
      <Section label="speed">
        <Field label="speed">
          <Slider min={1} max={8} step={1} value={s.settings.speed} onChange={v => send(set("snake", "speed", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Choice
          label="design"
          mode="cycle"
          options={opts(DESIGNS)}
          value={s.settings.design}
          onChange={v => send(set("snake", "design", v))}
        />
      </Section>
    </Stack>
  )
}
