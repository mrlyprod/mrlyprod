import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { SURFACES, SKINS, DESIGNS_SOLID as DESIGNS } from "../../components/options.ts"
import { call, set } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

const OPPONENTS = ["off", "random"]
const EMOJI: Record<string, string> = { x: "❌", o: "⭕" }

type Sprite = { rows: number[][]; palette: string[] }

type State = {
  score: number
  steps: number
  over: boolean
  board: (string | null)[][]
  sprites: (Sprite | null)[]
  winner: string | null
  turn: string
  settings: { opponent: string; surface: string; skin: string; design: string }
  frame: { rows: number[][]; palette: string[] }
}

function face(s: State, mark: string, sprite: Sprite | null, i: number): Node | undefined {
  if (s.settings.skin === "emojis") return <symbol key="face" as="emoji" value={EMOJI[mark] ?? mark} />
  if (s.settings.skin === "digits") return <symbol key="face" as="glyph" value={mark.toUpperCase()} />
  if (sprite === null) return <symbol key="face" as="glyph" value={mark.toUpperCase()} />
  return <canvas key="face" handle={`ttt-${i}`} rows={sprite.rows} palette={sprite.palette} />
}

export function ttt(state: unknown, _send: Send): Node {
  const s = state as State
  const grid = s.settings.surface === "grid"
  const marks = s.board.flat()
  const status = s.winner === null ? "draw" : `winner ${s.winner}`
  return (
    <stack key="ttt">
      <card key="board">
        {grid
          ? <grid key="grid" cols={3}>
              {marks.map((mark, i) =>
                mark === null ? (
                  <cell key={`c-${i}`} call={s.over ? undefined : call("ttt.place", { cell: i })} />
                ) : (
                  <cell key={`c-${i}`}>{face(s, mark, s.sprites[i] ?? null, i)}</cell>
                ),
              )}
            </grid>
          : <Board app="ttt" rows={s.frame.rows} palette={s.frame.palette} />}
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
