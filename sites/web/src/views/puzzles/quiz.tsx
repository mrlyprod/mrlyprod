import {
  Board,
  Box,
  Button,
  Caption,
  Cell,
  Choice,
  Field,
  Grid,
  Section,
  Slider,
  Stack,
} from "mrlyui"
import { call, set } from "../../builders"
import { GameOver } from "../../components/GameOver"
import { SURFACES, opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { Face } from "../../eyes/Face"
import { paint, visual } from "../../eyes/skin"
import { useSend } from "../../send"
import type { Cells as Deck, Visual } from "../../types"

const SKINS = ["tiles", "digits"]

type State = {
  score: number
  steps: number
  over: boolean
  won: boolean
  position: number
  total: number
  options: string[]
  settings: { options: number; length: number; surface: string; skin: string }
  cells: Deck
}

function inked(v: Visual): boolean {
  const f = v.face
  return f !== undefined && f.as !== "sprite" && f.value !== undefined && f.value !== ""
}

export function Quiz({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const grid = s.settings.surface === "grid"
  const id = s.cells.ids[0]?.[0] ?? 0
  const tile = visual("quiz", s.cells, id)
  const cols = Math.min(3, Math.max(1, s.options.length))

  return (
    <Stack>
      <Box>
        {grid && inked(tile) ? (
          <Board cols={1} rows={1}>
            <Cell bg={paint(tile, s.cells)}>
              <Face visual={tile} />
            </Cell>
          </Board>
        ) : (
          <Cells app="quiz" cells={s.cells} />
        )}
      </Box>
      {s.over ? (
        <GameOver
          app="quiz"
          emoji={s.won ? "🏆" : "❓"}
          status={s.won ? `cleared · score ${s.score}` : `wrong answer · score ${s.score}`}
        />
      ) : (
        <Box>
          <Grid cols={cols}>
            {s.options.map(text => (
              <Button key={text} onClick={() => send(call("quiz.answer", { text }))}>
                {text}
              </Button>
            ))}
          </Grid>
        </Box>
      )}
      <Box>
        {s.over ? null : <Caption>{`score ${s.score} · ${s.position} / ${s.total}`}</Caption>}
        <Shot />
      </Box>
      <Section label="rules">
        <Field label="options">
          <Slider min={2} max={8} step={1} value={s.settings.options} onChange={v => send(set("quiz", "options", v))} />
        </Field>
        <Field label="length">
          <Slider min={2} max={32} step={1} value={s.settings.length} onChange={v => send(set("quiz", "length", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Choice
          label="surface"
          mode="row"
          options={opts(SURFACES)}
          value={s.settings.surface}
          onChange={v => send(set("quiz", "surface", v))}
        />
        <Choice
          label="skin"
          mode="row"
          options={opts(SKINS)}
          value={s.settings.skin}
          onChange={v => send(set("quiz", "skin", v))}
        />
      </Section>
    </Stack>
  )
}
