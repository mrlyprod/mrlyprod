import {
  Board,
  Box,
  Caption,
  Cell,
  Choice,
  Field,
  Section,
  Slider,
  Stack,
} from "mrlyui"
import { set } from "../../builders"
import { DPad } from "../../components/DPad"
import { GameOver } from "../../components/GameOver"
import { DESIGNS_SOLID as DESIGNS, SKINS, SURFACES, opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { Face } from "../../eyes/Face"
import { paint, visual } from "../../eyes/skin"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

type State = {
  score: number
  steps: number
  over: boolean
  last_spawn: [number, number] | null
  last_merges: [number, number][]
  settings: { grid: number; surface: string; skin: string; design: string }
  cells: Deck
}

function fresh(s: State, r: number, c: number): boolean {
  if (s.last_spawn !== null && s.last_spawn[0] === r && s.last_spawn[1] === c) return true
  return s.last_merges.some(([mr, mc]) => mr === r && mc === c)
}

export function Twenty48({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const grid = s.settings.surface === "grid"
  const across = s.cells.ids[0]?.length ?? s.settings.grid
  const down = s.cells.ids.length

  const tile = (id: number, r: number, c: number) => {
    const v = visual("twenty48", s.cells, id)
    const nonce = fresh(s, r, c) ? `hot-${s.steps}` : "cold"
    return (
      <Cell key={`${r}-${c}`} bg={paint(v, s.cells)}>
        <Face key={nonce} visual={v} />
      </Cell>
    )
  }

  return (
    <Stack>
      <Box>
        <Caption>{`🔢 ${s.score} · steps ${s.steps}`}</Caption>
        {grid ? (
          <Board cols={across} rows={down}>
            {s.cells.ids.flatMap((row, r) => row.map((id, c) => tile(id, r, c)))}
          </Board>
        ) : (
          <Cells app="twenty48" cells={s.cells} />
        )}
      </Box>
      {s.over ? <GameOver app="twenty48" emoji="🔢" status={`score ${s.score}`} /> : null}
      <Box>
        {s.over ? null : <DPad app="twenty48" verb="slide" />}
        <Shot />
      </Box>
      <Section label="rules">
        <Field label="grid">
          <Slider min={2} max={8} step={1} value={s.settings.grid} onChange={v => send(set("twenty48", "grid", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Choice
          label="surface"
          mode="row"
          options={opts(SURFACES)}
          value={s.settings.surface}
          onChange={v => send(set("twenty48", "surface", v))}
        />
        <Choice
          label="skin"
          mode="row"
          options={opts(SKINS)}
          value={s.settings.skin}
          onChange={v => send(set("twenty48", "skin", v))}
        />
        <Choice
          label="design"
          mode="cycle"
          options={opts(DESIGNS)}
          value={s.settings.design}
          onChange={v => send(set("twenty48", "design", v))}
        />
      </Section>
    </Stack>
  )
}
