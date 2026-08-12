import {
  Board,
  Box,
  Button,
  Caption,
  Cell,
  Choice,
  Field,
  Section,
  Slider,
  Stack,
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

const FLAG = 11

type State = {
  score: number
  steps: number
  over: boolean
  won: boolean | null
  tool: string
  remaining: number
  flags: boolean[][]
  settings: { cols: number; rows: number; mines: number; surface: string; skin: string; design: string }
  cells: Deck
}

function inked(v: Visual): boolean {
  const f = v.face
  return f !== undefined && f.as !== "sprite" && f.value !== undefined && f.value !== ""
}

export function Mines({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const grid = s.settings.surface === "grid"
  const across = s.cells.ids[0]?.length ?? s.settings.cols
  const down = s.cells.ids.length
  const digging = s.tool === "dig"

  const tile = (id: number, r: number, c: number) => {
    const flagged = s.flags[r]?.[c] ?? false
    const v = visual("mines", s.cells, id)
    if (id === 0) {
      const verb = flagged || s.tool === "flag" ? "mines.flag" : "mines.reveal"
      return (
        <Cell
          key={`${r}-${c}`}
          bg={paint(v, s.cells)}
          onClick={s.over ? undefined : () => send(call(verb, { x: c, y: r }))}
        >
          {flagged ? <Face visual={visual("mines", s.cells, FLAG)} /> : null}
        </Cell>
      )
    }
    if (inked(v)) {
      return (
        <Cell key={`${r}-${c}`} bg={paint(v, s.cells)}>
          <Face visual={v} />
        </Cell>
      )
    }
    return (
      <Cell key={`${r}-${c}`}>
        <Cells app="mines" cells={{ ...s.cells, ids: [[id]] }} />
      </Cell>
    )
  }

  return (
    <Stack>
      <Box>
        <Caption>{`💣 ${s.remaining} · moves ${s.steps}`}</Caption>
        {grid ? (
          <Board cols={across} rows={down}>
            {s.cells.ids.flatMap((row, r) => row.map((id, c) => tile(id, r, c)))}
          </Board>
        ) : (
          <Cells
            app="mines"
            cells={s.cells}
            grid={[across, down]}
            onTap={s.over ? undefined : (x, y) => send(call(digging ? "mines.reveal" : "mines.flag", { x, y }))}
          />
        )}
      </Box>
      {s.over ? (
        <GameOver
          app="mines"
          emoji={s.won === true ? "🏆" : "💣"}
          status={s.won === true ? `cleared · ${s.score} revealed` : "boom"}
        />
      ) : null}
      <Box>
        {s.over ? null : (
          <Button
            active={!digging}
            onClick={() => send(call("mines.tool", { tool: digging ? "flag" : "dig" }))}
          >
            {digging ? "⛏ dig" : "⛳ flag"}
          </Button>
        )}
        <Shot />
      </Box>
      <Section label="rules">
        <Field label="cols">
          <Slider min={4} max={30} step={1} value={s.settings.cols} onChange={v => send(set("mines", "cols", v))} />
        </Field>
        <Field label="rows">
          <Slider min={4} max={30} step={1} value={s.settings.rows} onChange={v => send(set("mines", "rows", v))} />
        </Field>
        <Field label="mines">
          <Slider min={1} max={200} step={1} value={s.settings.mines} onChange={v => send(set("mines", "mines", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Choice
          label="surface"
          mode="row"
          options={opts(SURFACES)}
          value={s.settings.surface}
          onChange={v => send(set("mines", "surface", v))}
        />
        <Choice
          label="skin"
          mode="row"
          options={opts(SKINS)}
          value={s.settings.skin}
          onChange={v => send(set("mines", "skin", v))}
        />
        <Choice
          label="design"
          mode="cycle"
          options={opts(DESIGNS)}
          value={s.settings.design}
          onChange={v => send(set("mines", "design", v))}
        />
      </Section>
    </Stack>
  )
}
