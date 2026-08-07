import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { SURFACES, SKINS, DESIGNS_SOLID as DESIGNS } from "../../components/options.ts"
import { call, set } from "../../builders.ts"
import { face, visual } from "../../cells.tsx"
import { h } from "../../jsx.ts"
import type { Cells, Node, Send } from "../../types.ts"

const OPPONENTS = ["off", "random"]

type State = {
  score: number
  steps: number
  over: boolean
  board: (string | null)[][]
  winner: string | null
  turn: string
  settings: { opponent: string; surface: string; skin: string; design: string }
  cells: Cells
}

function mark(s: State, id: number, i: number): Node {
  const worn = face(visual("ttt", s.cells, id))
  if (worn !== undefined) return <cell key={`c-${i}`}>{worn}</cell>
  return (
    <cell key={`c-${i}`}>
      <canvas key="face" handle={`ttt-${i}`} cells={{ app: "ttt", ...s.cells, ids: [[id]] }} />
    </cell>
  )
}

export function ttt(state: unknown, _send: Send): Node {
  const s = state as State
  const grid = s.settings.surface === "grid"
  const marks = s.cells.ids.flat()
  const status = s.winner === null ? "draw" : `winner ${s.winner}`
  return (
    <stack key="ttt">
      <card key="board">
        {grid
          ? <grid key="grid" cols={3}>
              {marks.map((id, i) =>
                id === 0 ? (
                  <cell key={`c-${i}`} call={s.over ? undefined : call("ttt.place", { cell: i })} />
                ) : (
                  mark(s, id, i)
                ),
              )}
            </grid>
          : <Board app="ttt" cells={s.cells} />}
      </card>
      {s.over && <GameOver app="ttt" emoji="⭕" status={status} />}
      <card key="meter">
        {!s.over && <text key="meter" role="note">{`${s.turn}'s turn`}</text>}
        <Shot />
      </card>
      <Section keyName="rules" label="rules">
        <choice key="opponent" value={s.settings.opponent} options={OPPONENTS} call={set("ttt", "opponent")} arg="value" label="opponent" mode="row" />
      </Section>
      <Section keyName="look" label="look">
        <choice key="surface" value={s.settings.surface} options={SURFACES} call={set("ttt", "surface")} arg="value" label="surface" mode="row" />
        <choice key="skin" value={s.settings.skin} options={SKINS} call={set("ttt", "skin")} arg="value" label="skin" mode="row" />
        <choice key="design" value={s.settings.design} options={DESIGNS} call={set("ttt", "design")} arg="value" label="design" mode="cycle" />
      </Section>
    </stack>
  )
}
