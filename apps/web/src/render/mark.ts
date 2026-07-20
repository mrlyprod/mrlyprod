import { mark } from "../kernel.ts"
import type { Mark } from "../types.ts"
import { tint } from "./paint.ts"

type Cell = {
  ctx: CanvasRenderingContext2D
  style: CSSStyleDeclaration
  held: Map<number, string>
  idx: number
  x: number
  y: number
}

type Doodle = (cell: Cell) => void

let cache: Mark | null = null

function frames(): Mark {
  cache ??= mark()
  return cache
}

const squares: Doodle = ({ ctx, style, held, idx, x, y }) => {
  const hue = tint(held.get(idx))
  held.set(idx, hue)
  ctx.fillStyle = style.getPropertyValue(`--c-${hue}`)
  ctx.fillRect(x, y, 1, 1)
}

const doodles: Record<string, Doodle> = { squares }

export function markCanvas(doodle = "squares"): HTMLCanvasElement {
  const face = frames()
  const el = document.createElement("canvas")
  el.className = "mark"
  el.width = face.cols
  el.height = face.rows
  const ctx = el.getContext("2d")
  const held = new Map<number, string>()
  const draw = doodles[doodle] ?? squares
  let at = 0
  let seen = true
  const eye = new IntersectionObserver(entries => {
    seen = entries[0]?.isIntersecting ?? true
  })
  eye.observe(el)
  const timer = setInterval(() => {
    if (!el.isConnected) {
      clearInterval(timer)
      eye.disconnect()
      return
    }
    if (!seen || ctx === null || face.frames.length === 0) return
    const lit = face.frames[at % face.frames.length] as number[]
    at += 1
    ctx.clearRect(0, 0, face.cols, face.rows)
    const style = getComputedStyle(el)
    const on = new Set(lit)
    for (const idx of held.keys()) if (!on.has(idx)) held.delete(idx)
    for (const idx of lit) draw({ ctx, style, held, idx, x: idx % face.cols, y: Math.floor(idx / face.cols) })
  }, 1000 / face.fps)
  return el
}
