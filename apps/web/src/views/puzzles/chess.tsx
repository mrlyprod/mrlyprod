import { useState } from "react"
import {
  Board,
  Box,
  Button,
  Caption,
  Cell,
  Choice,
  Field,
  Input,
  Section,
  Slider,
  Stack,
  Toggle,
} from "mrlyui"
import { call, set } from "../../builders"
import { GameOver } from "../../components/GameOver"
import { SURFACES, opts } from "../../components/options"
import { Shot } from "../../components/Shot"
import { Bits } from "../../eyes/Bits"
import { Cells } from "../../eyes/Cells"
import { Face } from "../../eyes/Face"
import { visual } from "../../eyes/skin"
import { useSend } from "../../send"
import type { Cells as Deck } from "../../types"

const BLOCK = 13

const LIGHT = "#f0d9b1"

const DARK = "#b58863"

const LIGHT_LAST = "#f0dc82"

const DARK_LAST = "#cbaa4e"

const DOT = [[0, 0, 0], [0, 1, 0], [0, 0, 0]]

const SKINS = ["digits", "emojis"]

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
  moves: { from: string; to: string }[]
  last_move: { from: string; to: string } | null
  cells: Deck
}

function pen(s: State, id: number): string {
  const tint = visual("chess", s.cells, id).face?.tint
  if (tint === undefined || tint === "ink") return "#ffffff"
  return s.cells.pens[tint.pen] ?? "#ffffff"
}

export function Chess({ state }: { state: unknown }) {
  const s = state as State
  const send = useSend()
  const [layout, setLayout] = useState(s.settings.layout)
  const ranks = s.board.length
  const files = s.board[0]?.length ?? 0
  const status = s.winner === "draw" ? "stalemate · draw" : `checkmate · ${s.winner} wins`

  const piece = (id: number, nonce: string) => {
    const v = visual("chess", s.cells, id)
    if (v.face?.as !== "sprite") return <Face key={`p${nonce}`} visual={v} />
    return <Bits key={`p${nonce}`} rows={v.face.rows ?? []} palette={["transparent", pen(s, id)]} crisp />
  }

  const dot = () => {
    const team = s.turn === "white" ? 1 : 7
    return <Bits key="dot" rows={DOT} palette={["transparent", pen(s, team)]} crisp />
  }

  const square = (x: number, y: number) => {
    const name = `${String.fromCharCode(97 + x)}${ranks - y}`
    const letter = s.board[y]?.[x] ?? null
    const id = s.cells.ids[y]?.[x] ?? 0
    const target = s.targets.includes(name)
    const picked = s.selected === name
    const last = s.last_move !== null && (s.last_move.from === name || s.last_move.to === name)
    const dusk = (x + y) % 2 === 1
    const on = picked || (target && letter !== null)
    const moved = s.last_move !== null && s.last_move.to === name
    const royal = letter !== null && letter.toLowerCase() === "k" && (s.turn === "white") === (letter === "K")
    const nonce = (s.check && royal) || moved ? String(s.steps) : ""
    const bg = last ? (dusk ? DARK_LAST : LIGHT_LAST) : dusk ? DARK : LIGHT
    return (
      <Cell
        key={name}
        on={on}
        bg={on ? undefined : bg}
        onClick={s.over ? undefined : () => send(call("chess.select", { square: name }))}
      >
        {id % BLOCK > 0 ? piece(id, nonce) : target ? dot() : null}
      </Cell>
    )
  }

  const grid = () => {
    const spots = []
    for (let y = 0; y < ranks; y++) {
      for (let x = 0; x < files; x++) spots.push(square(x, y))
    }
    return spots
  }

  return (
    <Stack>
      <Box>
        <Caption>{s.over ? status : `${s.turn} to move${s.check ? " · check" : ""}`}</Caption>
        {s.settings.surface === "grid" ? (
          <Board cols={files} rows={ranks}>
            {grid()}
          </Board>
        ) : (
          <Cells
            app="chess"
            cells={s.cells}
            grid={[files, ranks]}
            onTap={s.over ? undefined : (x, y) => send(call("chess.select", { x, y }))}
          />
        )}
      </Box>
      {s.over ? <GameOver app="chess" emoji={s.winner === "draw" ? "🤝" : "👑"} status={status} /> : null}
      <Box>
        {s.over ? null : <Caption>{`ply ${s.steps} · ${s.moves.length} legal moves`}</Caption>}
        <Shot />
      </Box>
      <Section label="rules">
        <Field label="layout" hint="ranks from black to white, digits skip files">
          <Input value={layout} onChange={setLayout} />
        </Field>
        <Button onClick={() => send(set("chess", "layout", layout))}>deal</Button>
        <Field label="obfuscate" hint="hide which piece is which">
          <Toggle value={s.settings.obfuscate} onChange={v => send(set("chess", "obfuscate", v))} />
        </Field>
        <Field label="reskin" hint="plies between reskins, 0 never">
          <Slider min={0} max={50} step={1} value={s.settings.reskin} onChange={v => send(set("chess", "reskin", v))} />
        </Field>
      </Section>
      <Section label="look">
        <Choice
          label="surface"
          mode="row"
          options={opts(SURFACES)}
          value={s.settings.surface}
          onChange={v => send(set("chess", "surface", v))}
        />
        <Choice
          label="skin"
          mode="row"
          options={opts(SKINS)}
          value={s.settings.skin}
          onChange={v => send(set("chess", "skin", v))}
        />
      </Section>
    </Stack>
  )
}
