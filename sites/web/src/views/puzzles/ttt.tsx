import {
  Board,
  Box,
  Caption,
  Cell,
  Choice,
  Section,
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

const OPPONENTS = ["off", "random"]

type State = {
  score: number
  steps: number
  over: boolean
  board: (string | null)[][]
  winner: string | null
  turn: string
  settings: { opponent: string; surface: string; skin: string; design: string }
  cells: Deck
}

function inked(v: Visual): boolean {
  const f = v.face
  return f !== undefined && f.as !== "sprite" && f.value !== undefined && f.value !== ""
}

export function Ttt({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const grid = s.settings.surface === "grid"
  const marks = s.cells.ids.flat()
  const status = s.winner === null ? "draw" : `${s.winner} wins`

  const mark = (id: number, i: number) => {
    if (id === 0) {
      return <Cell key={i} onClick={s.over ? undefined : () => send(call("ttt.place", { cell: i }))} />
    }
    const v = visual("ttt", s.cells, id)
    if (inked(v)) {
      return (
        <Cell key={i} bg={paint(v, s.cells)}>
          <Face visual={v} />
        </Cell>
      )
    }
    return (
      <Cell key={i}>
        <Cells app="ttt" cells={{ ...s.cells, ids: [[id]] }} />
      </Cell>
    )
  }

  return (
    <Stack>
      <Box>
        {grid ? (
          <Board cols={3} rows={3}>
            {marks.map((id, i) => mark(id, i))}
          </Board>
        ) : (
          <Cells
            app="ttt"
            cells={s.cells}
            grid={[3, 3]}
            onTap={s.over ? undefined : (x, y) => send(call("ttt.place", { x, y }))}
          />
        )}
      </Box>
      {s.over ? <GameOver app="ttt" emoji={s.winner === null ? "🤝" : "🏆"} status={status} /> : null}
      <Box>
        {s.over ? null : <Caption>{`${s.turn}'s turn · move ${s.steps}`}</Caption>}
        <Shot />
      </Box>
      <Section label="rules">
        <Choice
          label="opponent"
          mode="row"
          options={opts(OPPONENTS)}
          value={s.settings.opponent}
          onChange={v => send(set("ttt", "opponent", v))}
        />
      </Section>
      <Section label="look">
        <Choice
          label="surface"
          mode="row"
          options={opts(SURFACES)}
          value={s.settings.surface}
          onChange={v => send(set("ttt", "surface", v))}
        />
        <Choice
          label="skin"
          mode="row"
          options={opts(SKINS)}
          value={s.settings.skin}
          onChange={v => send(set("ttt", "skin", v))}
        />
        <Choice
          label="design"
          mode="cycle"
          options={opts(DESIGNS)}
          value={s.settings.design}
          onChange={v => send(set("ttt", "design", v))}
        />
      </Section>
    </Stack>
  )
}
