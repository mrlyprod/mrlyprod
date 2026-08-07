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
  Toggle,
} from "mrlyui"
import { call, set } from "../../builders"
import { GameOver } from "../../components/GameOver"
import { DESIGNS_SOLID as DESIGNS, SKINS, SURFACES, opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Cells } from "../../eyes/Cells"
import { Face } from "../../eyes/Face"
import { paint, visual } from "../../eyes/skin"
import { useSend } from "../../send"
import type { Cells as Deck, Visual } from "../../types"

const VOID = 0
const BACK = 1

type State = {
  score: number
  steps: number
  over: boolean
  rounds: number
  look: number
  matched: boolean[][]
  settings: { pairs: number; cols: number; sudden: boolean; surface: string; skin: string; design: string }
  cells: Deck
}

function inked(v: Visual): boolean {
  const f = v.face
  return f !== undefined && f.as !== "sprite" && f.value !== undefined && f.value !== ""
}

export function Memory({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const grid = s.settings.surface === "grid"
  const live = !s.over && s.look === 0
  const across = s.cells.ids[0]?.length ?? s.settings.cols
  const down = s.cells.ids.length
  const status = s.look > 0 ? "look!" : `🍐 ${s.score} · round ${s.rounds + 1} · flips ${s.steps}`

  const tile = (id: number, r: number, c: number) => {
    const done = s.matched[r]?.[c] ?? false
    const v = visual("memory", s.cells, id)
    if (id === VOID) return <Cell key={`${r}-${c}`} />
    if (id === BACK) {
      return (
        <Cell
          key={`${r}-${c}`}
          bg={paint(v, s.cells)}
          onClick={live ? () => send(call("memory.flip", { x: c, y: r })) : undefined}
        />
      )
    }
    const nonce = done ? "held" : `turn-${s.steps}`
    if (inked(v)) {
      return (
        <Cell key={`${r}-${c}`} on={done} bg={paint(v, s.cells)}>
          <Face key={nonce} visual={v} />
        </Cell>
      )
    }
    return (
      <Cell key={`${r}-${c}`} on={done}>
        <Cells app="memory" cells={{ ...s.cells, ids: [[id]] }} />
      </Cell>
    )
  }

  return (
    <Stack>
      <Box>
        <Caption>{status}</Caption>
        {grid ? (
          <Board cols={across} rows={down}>
            {s.cells.ids.flatMap((row, r) => row.map((id, c) => tile(id, r, c)))}
          </Board>
        ) : (
          <Cells
            app="memory"
            cells={s.cells}
            grid={[across, down]}
            onTap={live ? (x, y) => send(call("memory.flip", { x, y })) : undefined}
          />
        )}
      </Box>
      {s.over ? <GameOver app="memory" emoji="🧠" status={`${s.score} pairs · ${s.rounds} rounds`} /> : null}
      <Box>
        <Shot />
      </Box>
      <Section label="rules">
        <Field label="pairs">
          <Slider min={2} max={8} step={1} value={s.settings.pairs} onChange={v => send(set("memory", "pairs", v))} />
        </Field>
        <Field label="cols">
          <Slider min={2} max={8} step={1} value={s.settings.cols} onChange={v => send(set("memory", "cols", v))} />
        </Field>
        <Field label="sudden death" hint="one miss ends the run">
          <Toggle value={s.settings.sudden} onChange={v => send(set("memory", "sudden", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Choice
          label="surface"
          mode="row"
          options={opts(SURFACES)}
          value={s.settings.surface}
          onChange={v => send(set("memory", "surface", v))}
        />
        <Choice
          label="skin"
          mode="row"
          options={opts(SKINS)}
          value={s.settings.skin}
          onChange={v => send(set("memory", "skin", v))}
        />
        <Choice
          label="design"
          mode="cycle"
          options={opts(DESIGNS)}
          value={s.settings.design}
          onChange={v => send(set("memory", "design", v))}
        />
      </Section>
    </Stack>
  )
}
