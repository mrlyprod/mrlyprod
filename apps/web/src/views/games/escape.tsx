import {
  Box,
  Caption,
  Choice,
  Field,
  Section,
  Slider,
  Stack,
} from "mrlyui"
import { set } from "../../builders"
import { DPad } from "../../components/DPad"
import { GameOver } from "../../components/GameOver"
import { DESIGNS_SOLID as DESIGNS, opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

const MAPS = ["random", "0", "1", "2"]

type State = {
  score: number
  steps: number
  level: number
  over: boolean
  escaped: boolean | null
  settings: {
    map: string
    ghost_ratio: number
    speed: number
    design: string
  }
  cells: Deck
}

export function Escape({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const done = s.escaped === true ? `escaped · ate ${s.score}` : `caught · level ${s.level}`
  return (
    <Stack>
      <Box>
        <Caption>{`🚪 level ${s.level} · ate ${s.score}`}</Caption>
        <Cells app="escape" cells={s.cells} />
      </Box>
      {s.over ? <GameOver app="escape" emoji={s.escaped === true ? "🚪" : "👻"} status={done} /> : null}
      <Box>
        {s.over ? null : <DPad app="escape" verb="turn" />}
        <Shot />
      </Box>
      <Section label="rules">
        <Choice
          label="map"
          mode="cycle"
          options={opts(MAPS)}
          value={s.settings.map}
          onChange={v => send(set("escape", "map", v))}
        />
        <Field label="ghosts">
          <Slider
            min={1}
            max={4}
            step={1}
            value={s.settings.ghost_ratio}
            onChange={v => send(set("escape", "ghost_ratio", v))}
          />
        </Field>
      </Section>
      <Section label="speed">
        <Field label="speed">
          <Slider min={1} max={4} step={1} value={s.settings.speed} onChange={v => send(set("escape", "speed", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Choice
          label="design"
          mode="cycle"
          options={opts(DESIGNS)}
          value={s.settings.design}
          onChange={v => send(set("escape", "design", v))}
        />
      </Section>
    </Stack>
  )
}
