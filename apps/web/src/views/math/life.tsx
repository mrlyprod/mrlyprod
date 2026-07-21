import { call, pickColor, pickTile, setter } from "../../builders.ts"
import { Board } from "../../components/Board.tsx"
import { Shot } from "../../components/Shot.tsx"
import { h } from "../../jsx.ts"
import type { Flip, Node, Send } from "../../types.ts"

const PRESETS = ["seed", "clear", "soup"]
const CLASSICS = ["glider", "pulsar", "pentomino"]
const SEQS = ["evens", "odds", "primes", "fibonacci"]

type Work = unknown

type State = {
  frame: { rows: number[][]; palette: string[] }
  strip?: Flip[]
  generation: number
  population: number
  entropy: number
  fate: string | null
  period: number
  running: boolean
  cursor: number
  length: number
  max_neighbors: number
  settings: {
    size: number
    wrap: boolean
    speed: number
    tiling: number
    padding: number
    density: number
    birth: number[]
    survive: number[]
    zeros: boolean
    ones: boolean
    seed: Work
    mask: Work
    fill: string
    void: string
  }
}

const turn = setter("life")

function status(s: State): string {
  const base = `gen ${s.generation} · pop ${s.population} · H ${s.entropy}`
  if (s.fate === null) return base
  return s.period > 1 ? `${base} · ${s.fate} · p${s.period}` : `${base} · ${s.fate}`
}

function transport(s: State): Node[] {
  const settled = s.fate !== null && s.cursor === s.length - 1
  return [
    s.cursor > 0 ? <button key="start" call={call("life.start")}>⏮</button> : null,
    s.cursor > 0 ? <button key="back" call={call("life.back")}>◀</button> : null,
    s.running
      ? <button key="pause" call={call("life.run", { on: false })}>pause</button>
      : <button key="run" call={call("life.run", { on: true })}>run</button>,
    !settled ? <button key="step" call={call("life.step")}>▶</button> : null,
    s.cursor < s.length - 1 ? <button key="end" call={call("life.end")}>⏭</button> : null,
  ].filter((node): node is Node => node !== null)
}

function chips(s: State, which: "birth" | "survive"): Node {
  const on = which === "birth" ? s.settings.birth : s.settings.survive
  const counts = Array.from({ length: s.max_neighbors + 1 }, (_, n) => n)
  return (
    <grid key={`${which}-chips`} cols={s.max_neighbors + 1}>
      {counts.map(n => (
        <toggle key={`${which}${n}`} on={on.includes(n)} call={call("life.rule", { which, n })} arg="on" label={String(n)} />
      ))}
    </grid>
  )
}

function fills(which: "birth" | "survive"): Node {
  return (
    <grid key={`${which}-fills`} cols={SEQS.length}>
      {SEQS.map(seq => (
        <button key={`${which}-${seq}`} call={call("life.fill", { which, seq })}>{seq}</button>
      ))}
    </grid>
  )
}

export function life(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="life">
      <card key="board">
        <Board
          app="life"
          rows={s.frame.rows}
          palette={s.frame.palette}
          strip={s.strip}
          grid={[s.settings.size, s.settings.size]}
          drag={s.running ? undefined : call("life.paint")}
        />
      </card>
      <card key="transport">
        {transport(s)}
        <Shot />
      </card>
      <card key="status">
        <text key="meter" role="note">{status(s)}</text>
      </card>
      <card key="presets">
        <text key="presets-label" role="label">seed</text>
        {PRESETS.map(p => (
          <button key={p} call={call("life.reset", { pattern: p })}>{p}</button>
        ))}
        <range key="density" value={s.settings.density} min={1} max={99} step={1} call={turn("density")} arg="value" label="density" />
        {CLASSICS.map(p => (
          <button key={p} call={call("life.reset", { pattern: p })}>{p}</button>
        ))}
      </card>
      <card key="seed">
        <text key="seed-label" role="label">seed tile</text>
        <button key="pick-seed" call={pickTile("life", "seed")}>pick seed</button>
        <range key="tiling" value={s.settings.tiling} min={1} max={8} step={1} call={turn("tiling")} arg="value" label="tiling" />
        <range key="padding" value={s.settings.padding} min={0} max={8} step={1} call={turn("padding")} arg="value" label="padding" />
      </card>
      <card key="mask">
        <text key="mask-label" role="label">neighborhood</text>
        <button key="pick-mask" call={pickTile("life", "mask")}>pick mask</button>
      </card>
      <card key="rules">
        <text key="birth-label" role="label">birth</text>
        {chips(s, "birth")}
        {fills("birth")}
        <text key="survive-label" role="label">survive</text>
        {chips(s, "survive")}
        {fills("survive")}
        <toggle key="zeros" on={s.settings.zeros} call={turn("zeros")} arg="value" label="zeros" />
        <toggle key="ones" on={s.settings.ones} call={turn("ones")} arg="value" label="ones" />
        <toggle key="wrap" on={s.settings.wrap} call={turn("wrap")} arg="value" label="wrap" />
      </card>
      <card key="board-speed">
        <text key="bs-label" role="label">board</text>
        <range key="size" value={s.settings.size} min={8} max={64} step={1} call={turn("size")} arg="value" label="size" />
        <range key="speed" value={s.settings.speed} min={1} max={32} step={1} call={turn("speed")} arg="value" label="speed" />
      </card>
      <card key="colors">
        <text key="colors-label" role="label">colors</text>
        <button key="fill" call={pickColor("life", "fill", s.settings.fill)}>{`fill · ${s.settings.fill}`}</button>
        <button key="void" call={pickColor("life", "void", s.settings.void)}>{`void · ${s.settings.void}`}</button>
      </card>
    </stack>
  )
}
