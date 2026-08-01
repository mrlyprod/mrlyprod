import { useEffect, useRef, useState } from "react"
import { read, write } from "mrlydom"
import * as gpu from "mrlygpu"
import init, { brick, pose, shader, sheet, turn, uniforms } from "../../../../pkgs/mrlyjs/math/pkg/mrlyjs_math.js"
import wasm from "../../../../pkgs/mrlyjs/math/pkg/mrlyjs_math_bg.wasm?url"
import { place } from "../lib/orders"
import type { Order } from "../lib/orders"
import { BUILD, DESIGNED, LEAD, PRESET, PRICES } from "../lib/site"
import type { Tier } from "../lib/site"

// STATE

type Mode = "math" | "bricks" | "sheets"

const MODES: Mode[] = ["math", "bricks", "sheets"]
const DESIGNS = ["carpet", "net", "xtree", "ytree", "ztree"]
const NUMBERS = [3, 5, 7, 9]
const LEVELS = [1, 2, 3]
const CELLS = 32
const DRAG = 128
const PITCH = 56
const NEAR = 8
const FAR = 32
const TRIES = 240
const TAU = Math.PI * 2

const state = {
  mode: "bricks" as Mode,
  design: "carpet",
  number: 3,
  level: 2,
  cols: 12,
  rows: 17,
  bore: 16,
  thick: 8,
  edges: true,
}

const rig = { yaw: 0, pitch: 0, dist: 12 }
let flat = true
let span = 256
let shape: Float32Array = new Float32Array(0)
let sig = ""
let queued = 0
let tries = 0

// COLOUR

function rgb(css: string, fallback: [number, number, number]): [number, number, number] {
  const raw = css.trim()
  if (raw.startsWith("#")) {
    const digits = raw.slice(1)
    const wide = digits.length >= 6
    const at = (i: number) => parseInt(wide ? digits.slice(i * 2, i * 2 + 2) : (digits[i] ?? "0").repeat(2), 16)
    return [at(0), at(1), at(2)]
  }
  const parts = raw.match(/[\d.]+/g)
  if (parts === null || parts.length < 3) return fallback
  return [Math.round(Number(parts[0])), Math.round(Number(parts[1])), Math.round(Number(parts[2]))]
}

function token(name: string, fallback: [number, number, number]): [number, number, number] {
  return rgb(getComputedStyle(document.documentElement).getPropertyValue(name), fallback)
}

function board(): [number, number, number] {
  return rgb(getComputedStyle(document.body).backgroundColor, [255, 255, 255])
}

function hex(parts: [number, number, number]): string {
  return `#${parts.map(v => v.toString(16).padStart(2, "0")).join("")}`
}

// MESH

function stamp(): string {
  const ink = hex(token("--font-color", [0, 0, 0]))
  if (state.mode === "sheets") return `sheet:${state.cols}:${state.rows}:${state.bore}:${state.thick}:${state.edges}:${ink}`
  return `brick:${state.design}:${state.number}:${state.level}:${state.edges}:${ink}`
}

function build(): Float32Array {
  const [r, g, b] = token("--font-color", [0, 0, 0])
  const ink = new Uint8Array([r, g, b, 255])
  if (state.mode === "sheets") return sheet(state.cols, state.rows, state.bore, state.thick, state.edges, ink)
  return brick(state.design, state.number, state.level, state.edges, ink)
}

function shading(): Float32Array {
  const paper = new Uint8Array(board())
  const fill = new Uint8Array(token("--accent-color", [142, 142, 147]))
  return uniforms(rig.yaw, rig.pitch, rig.dist, 0, 0, flat, 255, paper, fill)
}

// PAINT

function paint(canvas: HTMLCanvasElement): boolean {
  const mark = stamp()
  if (mark !== sig) {
    shape = build()
    sig = mark
  }
  const shade = { program: "mesh", route: BUILD, mesh: sig }
  return gpu.draw(canvas, { rows: [], shade }, gpu.pull(shade))
}

function schedule(canvas: HTMLCanvasElement): void {
  if (queued !== 0) return
  queued = requestAnimationFrame(() => {
    queued = 0
    if (paint(canvas)) {
      tries = 0
      return
    }
    tries += 1
    if (tries < TRIES) schedule(canvas)
  })
}

// ORBIT

function orbit(canvas: HTMLCanvasElement): void {
  let grip: { id: number; x: number; y: number; yaw: number; pitch: number } | null = null
  const settle = (): void => {
    if (grip === null) return
    const dyaw = Math.round(grip.yaw)
    const dpitch = Math.round(grip.pitch)
    grip = null
    gpu.steer(canvas, null)
    rig.yaw = (((rig.yaw + dyaw) % span) + span) % span
    rig.pitch = Math.max(-PITCH, Math.min(PITCH, rig.pitch + dpitch))
    schedule(canvas)
  }
  canvas.addEventListener("pointerdown", event => {
    canvas.setPointerCapture(event.pointerId)
    grip = { id: event.pointerId, x: event.clientX, y: event.clientY, yaw: 0, pitch: 0 }
  })
  canvas.addEventListener("pointermove", event => {
    if (grip === null || grip.id !== event.pointerId) return
    const rect = canvas.getBoundingClientRect()
    if (rect.width === 0) return
    grip.yaw += ((event.clientX - grip.x) / rect.width) * DRAG
    grip.pitch -= ((event.clientY - grip.y) / rect.width) * DRAG
    grip.x = event.clientX
    grip.y = event.clientY
    gpu.steer(canvas, { yaw: (grip.yaw * TAU) / span, pitch: (grip.pitch * TAU) / span, dist: 0 })
  })
  canvas.addEventListener("pointerup", settle)
  canvas.addEventListener("pointercancel", settle)
  canvas.addEventListener(
    "wheel",
    event => {
      event.preventDefault()
      rig.dist = Math.max(NEAR, Math.min(FAR, rig.dist + Math.sign(event.deltaY)))
      schedule(canvas)
    },
    { passive: false },
  )
}

// ORDER

function depth(): number {
  let level = state.level
  while (level > 1 && state.number ** level > CELLS) level -= 1
  return level
}

function bulk(): number {
  if (state.mode === "sheets") return state.cols * state.rows
  return state.number ** depth()
}

function fits(): Tier | null {
  const rows = PRICES[state.mode]
  if (rows === undefined) return null
  const want = bulk()
  return rows.find(tier => tier.max >= want) ?? null
}

function config(): Record<string, string | number | boolean> {
  if (state.mode === "sheets") {
    return { cols: state.cols, rows: state.rows, bore: state.bore, thick: state.thick, edges: state.edges }
  }
  return { design: state.design, number: state.number, level: depth(), edges: state.edges }
}

function entry(tier: Tier): Order {
  return { product: state.mode, tier: tier.id, label: tier.label, price: tier.price, config: config(), at: Date.now() }
}

// PRESET

function stored(): Mode {
  const held = read(PRESET)
  return MODES.find(mode => mode === held) ?? "bricks"
}

function remember(mode: Mode): void {
  write(PRESET, mode)
}

// MOUNT

async function start(): Promise<boolean> {
  if (navigator.gpu === undefined) return false
  const adapter = await navigator.gpu.requestAdapter().catch(() => null)
  if (adapter === null) return false
  await init({ module_or_path: wasm })
  const seed = pose()
  span = turn()
  rig.yaw = seed[0] ?? 32
  rig.pitch = seed[1] ?? 20
  rig.dist = seed[2] ?? 12
  flat = (seed[5] ?? 1) !== 0
  state.mode = stored()
  if (state.mode === "math") state.design = "carpet"
  gpu.init(
    { mesh: shader() },
    {
      geometry: () => shape,
      uniforms: () => shading(),
      board: () => hex(board()),
    },
  )
  return true
}

// CONTROLS

type Pickable = { name: string; values: (string | number)[]; held: string | number; set: (value: string) => void }

function Pick({ name, values, held, set }: Pickable) {
  return (
    <label>
      <span>{name}</span>
      <select value={String(held)} onChange={event => set(event.target.value)}>
        {values.map(value => (
          <option key={String(value)} value={String(value)}>
            {String(value)}
          </option>
        ))}
      </select>
    </label>
  )
}

type Slidable = { name: string; min: number; max: number; held: number; set: (value: number) => void }

function Slide({ name, min, max, held, set }: Slidable) {
  return (
    <label>
      <span>{name}</span>
      <input type="range" min={min} max={max} value={held} onChange={event => set(Number(event.target.value))} />
      <span className="val">{held}</span>
    </label>
  )
}

type Checkable = { name: string; held: boolean; set: (value: boolean) => void }

function Toggle({ name, held, set }: Checkable) {
  return (
    <label>
      <span>{name}</span>
      <input type="checkbox" checked={held} onChange={event => set(event.target.checked)} />
    </label>
  )
}

// BUILD

export function Build() {
  const [ready, setReady] = useState(false)
  const [, tick] = useState(0)
  const frame = useRef<HTMLCanvasElement | null>(null)

  useEffect(() => {
    let alive = true
    void start()
      .then(ok => {
        if (alive && ok) setReady(true)
      })
      .catch(() => {})
    return () => {
      alive = false
    }
  }, [])

  useEffect(() => {
    const node = frame.current
    if (node === null) return
    orbit(node)
    const watcher = new MutationObserver(() => schedule(node))
    watcher.observe(document.documentElement, { attributeFilter: ["class", "data-theme"] })
    tries = 0
    schedule(node)
    return () => {
      watcher.disconnect()
      if (queued !== 0) cancelAnimationFrame(queued)
      queued = 0
      gpu.drop(node)
    }
  }, [ready])

  const patch = (next: Partial<typeof state>): void => {
    Object.assign(state, next)
    const node = frame.current
    if (node !== null) schedule(node)
    tick(n => n + 1)
  }

  const swap = (mode: Mode): void => {
    if (mode === state.mode) return
    remember(mode)
    patch(mode === "math" ? { mode, design: "carpet" } : { mode })
  }

  if (!ready) {
    return (
      <div className="designer" id="designer">
        <p className="lead">The designer needs WebGPU.</p>
        <p className="fine">
          Try a recent Chrome, Edge or Safari, or <a href={`/${DESIGNED[1] ?? "bricks"}`}>read about the bricks</a>.
        </p>
      </div>
    )
  }
  const tier = fits()
  return (
    <div className="designer" id="designer">
      <canvas ref={frame} aria-label="preview" />
      <div className="rig">
        <div className="modes">
          {MODES.map(mode => (
            <button key={mode} type="button" aria-pressed={mode === state.mode} onClick={() => swap(mode)}>
              {mode}
            </button>
          ))}
        </div>
        {state.mode === "sheets" ? (
          <>
            <Slide name="cols" min={2} max={48} held={state.cols} set={cols => patch({ cols })} />
            <Slide name="rows" min={2} max={48} held={state.rows} set={rows => patch({ rows })} />
            <Slide name="bore" min={2} max={40} held={state.bore} set={bore => patch({ bore })} />
            <Slide name="thick" min={1} max={80} held={state.thick} set={thick => patch({ thick })} />
          </>
        ) : (
          <>
            <Pick name="design" values={DESIGNS} held={state.design} set={design => patch({ design })} />
            <Pick name="number" values={NUMBERS} held={state.number} set={value => patch({ number: Number(value) })} />
            <Pick name="level" values={LEVELS} held={state.level} set={value => patch({ level: Number(value) })} />
          </>
        )}
        <Toggle name="edges" held={state.edges} set={edges => patch({ edges })} />
      </div>
      <div className="order">
        <p className="lead">
          {tier === null || tier.url === "" ? "Pre-orders open soon." : `${tier.label} · €${String(tier.price)}`}
        </p>
        {tier !== null && tier.url !== "" && (
          <p className="act">
            <a className="build" href={tier.url} onClick={() => place(entry(tier))}>
              Pre-order
            </a>
          </p>
        )}
        <p className="fine">{LEAD}</p>
      </div>
    </div>
  )
}
