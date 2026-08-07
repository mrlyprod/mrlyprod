import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { SURFACES } from "../../components/options.ts"
import { call, set } from "../../builders.ts"
import { face as worn, visual } from "../../cells.tsx"
import { h } from "../../jsx.ts"
import type { Cells, Node, Send } from "../../types.ts"

type State = {
  steps: number
  over: boolean
  settings: { layout: string; obfuscate: boolean; reskin: number; surface: string; skin: string }
  turn: string
  check: boolean
  winner: string | null
  board: (string | null)[][]
  selected: string | null
  targets: string[]
  last_move: { from: string; to: string } | null
  cells: Cells
}

const BLOCK = 13
const BOARD = ["#f0d9b1", "#b58863"]
const LAST = ["#f0dc82", "#cbaa4e"]
const DOT = [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
const SKINS = ["digits", "emojis"]

function pen(s: State, id: number): string {
  const tint = visual("chess", s.cells, id).face?.tint
  if (tint === undefined || tint === "ink") return "#ffffff"
  return s.cells.pens[tint.pen] ?? "#ffffff"
}

const face = (s: State, id: number, nonce: string): Node | undefined => {
  const v = visual("chess", s.cells, id)
  if (v.face?.as !== "sprite") return worn(v, `piece${nonce}`)
  return <canvas key={`piece${nonce}`} handle="chess" rows={v.face.rows ?? []} palette={["transparent", pen(s, id)]} />
}

const dot = (s: State): Node => {
  const team = s.turn === "white" ? 1 : 7
  return <canvas key="dot" handle="chess" rows={DOT} palette={["transparent", pen(s, team)]} />
}

function board(s: State): Node {
  const ranks = s.board.length
  const files = s.board[0]?.length ?? 0
  const cells: Node[] = []
  for (let y = 0; y < ranks; y++) {
    for (let x = 0; x < files; x++) {
      const square = `${String.fromCharCode(97 + x)}${ranks - y}`
      const letter = s.board[y]?.[x] ?? null
      const id = s.cells.ids[y]?.[x] ?? 0
      const target = s.targets.includes(square)
      const picked = s.selected === square
      const last = s.last_move !== null && (s.last_move.from === square || s.last_move.to === square)
      const shade = (x + y) % 2
      const on = picked || (target && letter !== null)
      const moved = s.last_move !== null && s.last_move.to === square
      const royal = letter !== null && letter.toLowerCase() === "k" && (s.turn === "white") === (letter === "K")
      const nonce = (s.check && royal) || moved ? String(s.steps) : ""
      const child = id % BLOCK > 0 ? face(s, id, nonce) : target ? dot(s) : undefined
      cells.push(
        <cell
          key={square}
          call={s.over ? undefined : call("chess.select", { square })}
          on={on || undefined}
          bg={on ? undefined : last ? LAST[shade] : BOARD[shade]}
        >
          {child}
        </cell>,
      )
    }
  }
  return <grid key="squares" cols={files}>{cells}</grid>
}

export function chess(state: unknown, _send: Send): Node {
  const s = state as State
  const status = s.winner === "draw" ? "stalemate · draw" : `checkmate · ${s.winner} wins`
  const files = s.board[0]?.length ?? 0
  return (
    <stack key="chess">
      <card key="board">
        {s.settings.surface === "canvas"
          ? <Board app="chess" cells={s.cells} tap={s.over ? undefined : call("chess.select")} grid={[files, s.board.length]} />
          : board(s)}
      </card>
      {s.over && <GameOver app="chess" emoji="♟️" status={status} />}
      <card key="meter">
        {!s.over && <text key="meter" role="note">{`${s.turn} to move${s.check ? " · check" : ""} · ply ${s.steps}`}</text>}
        <Shot />
      </card>
      <Section keyName="rules" label="rules">
        <field key="layout" value={s.settings.layout} live={false} call={set("chess", "layout")} arg="value" label="layout" />
        <toggle key="obfuscate" on={s.settings.obfuscate} call={set("chess", "obfuscate")} arg="value" label="obfuscate" />
        <range key="reskin" value={s.settings.reskin} min={0} max={50} step={1} call={set("chess", "reskin")} arg="value" label="reskin" />
      </Section>
      <Section keyName="look" label="look">
        <choice key="surface" value={s.settings.surface} options={SURFACES} call={set("chess", "surface")} arg="value" label="surface" mode="row" />
        <choice key="skin" value={s.settings.skin} options={SKINS} call={set("chess", "skin")} arg="value" label="skin" mode="row" />
      </Section>
    </stack>
  )
}
