import {
  Box,
  Caption,
  Field,
  Section,
  Slider,
  Stack,
} from "mrlyui"
import { set } from "../../builders"
import { DPad } from "../../components/DPad"
import { GameOver } from "../../components/GameOver"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

type State = {
  score: number
  steps: number
  over: boolean
  settings: {
    board: number
    paddle: number
    block: number
    rows: number
    physics: number
    speed: number
  }
  cells: Deck
}

export function Tennis({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  return (
    <Stack>
      <Box>
        <Caption>{`🧱 ${s.score} · steps ${s.steps}`}</Caption>
        <Cells app="tennis" cells={s.cells} />
      </Box>
      {s.over ? <GameOver app="tennis" emoji="🎾" status={`score ${s.score}`} /> : null}
      <Box>
        {s.over ? null : <DPad app="tennis" verb="move" />}
        <Shot />
      </Box>
      <Section label="rules">
        <Field label="board">
          <Slider min={8} max={40} step={1} value={s.settings.board} onChange={v => send(set("tennis", "board", v))} />
        </Field>
        <Field label="paddle">
          <Slider min={2} max={10} step={1} value={s.settings.paddle} onChange={v => send(set("tennis", "paddle", v))} />
        </Field>
        <Field label="block">
          <Slider min={1} max={6} step={1} value={s.settings.block} onChange={v => send(set("tennis", "block", v))} />
        </Field>
        <Field label="rows">
          <Slider min={1} max={10} step={1} value={s.settings.rows} onChange={v => send(set("tennis", "rows", v))} />
        </Field>
        <Field label="physics" hint="how much bounce the paddle keeps">
          <Slider
            min={0.1}
            max={0.9}
            step={0.1}
            value={s.settings.physics / 1000}
            format={v => v.toFixed(1)}
            onChange={v => send(set("tennis", "physics", Math.round(v * 1000)))}
          />
        </Field>
      </Section>
      <Section label="speed">
        <Field label="speed">
          <Slider min={1} max={8} step={1} value={s.settings.speed} onChange={v => send(set("tennis", "speed", v))} />
        </Field>
      </Section>
    </Stack>
  )
}
