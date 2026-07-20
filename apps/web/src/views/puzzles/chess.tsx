import { GameOver } from "../../components/GameOver.tsx"
import { Section } from "../../components/Section.tsx"
import { Shot } from "../../components/Shot.tsx"
import { Board } from "../../components/Board.tsx"
import { SURFACES } from "../../components/options.ts"
import { call, set } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

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
  frame: { rows: number[][]; palette: string[] }
}

const PIECES: Record<string, string[]> = {
  p: ["00000", "00100", "01110", "00100", "00000"],
  n: ["01010", "10001", "00100", "10001", "01010"],
  b: ["10001", "01010", "00100", "01010", "10001"],
  r: ["00100", "00100", "11111", "00100", "00100"],
  q: ["10101", "01110", "11111", "01110", "10101"],
  k: ["00000", "01110", "01110", "01110", "00000"],
}

const EMOJIS: Record<string, [string, string]> = {
  p: ["♙", "♟"],
  n: ["♘", "♞"],
  b: ["♗", "♝"],
  r: ["♖", "♜"],
  q: ["♕", "♛"],
  k: ["♔", "♚"],
}

const BOARD = ["#f0d9b1", "#b58863"]
const LAST = ["#f0dc82", "#cbaa4e"]
const TEAMS = ["#ffffff", "#111111"]
const DOT = [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
const SKINS = ["digits", "emojis"]

const team = (letter: string) => (letter === letter.toUpperCase() ? 0 : 1)

const mask = (letter: string): number[][] =>
  (PIECES[letter.toLowerCase()] ?? []).map(row => [...row].map(ch => (ch === "1" ? 1 : 0)))

const face = (letter: string, nonce: string, skin: string): Node =>
  skin === "emojis"
    ? <symbol key={`piece${nonce}`} as="emoji" value={(EMOJIS[letter.toLowerCase()] ?? ["", ""])[team(letter)] ?? ""} />
    : <canvas key={`piece${nonce}`} handle="chess" rows={mask(letter)} palette={["transparent", TEAMS[team(letter)] ?? "#ffffff"]} />

const dot = (turn: string): Node => (
  <canvas key="dot" handle="chess" rows={DOT} palette={["transparent", turn === "white" ? "#ffffff" : "#111111"]} />
)

function board(s: State): Node {
  const ranks = s.board.length
  const files = s.board[0]?.length ?? 0
  const cells: Node[] = []
  for (let y = 0; y < ranks; y++) {
    for (let x = 0; x < files; x++) {
      const square = `${String.fromCharCode(97 + x)}${ranks - y}`
      const letter = s.board[y]?.[x] ?? null
      const target = s.targets.includes(square)
      const picked = s.selected === square
      const last = s.last_move !== null && (s.last_move.from === square || s.last_move.to === square)
      const shade = (x + y) % 2
      const on = picked || (target && letter !== null)
      const moved = s.last_move !== null && s.last_move.to === square
      const royal = letter !== null && letter.toLowerCase() === "k" && (s.turn === "white") === (letter === "K")
      const nonce = (s.check && royal) || moved ? String(s.steps) : ""
      const child = letter !== null ? face(letter, nonce, s.settings.skin) : target ? dot(s.turn) : undefined
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
          ? <Board app="chess" rows={s.frame.rows} palette={s.frame.palette} tap={s.over ? undefined : call("chess.select")} grid={[files, s.board.length]} />
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
