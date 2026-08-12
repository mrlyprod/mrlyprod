import { useCallback, useEffect, useRef } from "react"
import { Canvas } from "mrlyui"
import type { Point } from "mrlyui"
import paletteData from "../gen/palette.json"
import { visual } from "./skin"
import { dark } from "./theme"
import type { Cells as Deck, Palette, Pen, Visual } from "../types"

type Fact = Deck & { app: string }

const palette = paletteData as Palette

const GRAIN = 8

const STACK = `"Apple Color Emoji", "Segoe UI Emoji", "Noto Color Emoji", sans-serif`

// PAINT

function pick(fact: Fact, choice: string | Pen | undefined): string | undefined {
  if (choice === undefined) return undefined
  return typeof choice === "string" ? choice : fact.pens[choice.pen]
}

function draw(surface: HTMLCanvasElement, fact: Fact, night: boolean): void {
  const down = fact.ids.length
  const across = fact.ids[0]?.length ?? 0
  if (down === 0 || across === 0) return
  const rect = surface.getBoundingClientRect()
  if (rect.width === 0 || rect.height === 0) return
  const dpr = globalThis.devicePixelRatio || 1
  const width = Math.max(across, Math.round(rect.width * dpr))
  const height = Math.max(down, Math.round(rect.height * dpr))
  if (surface.width !== width) surface.width = width
  if (surface.height !== height) surface.height = height
  const ctx = surface.getContext("2d")
  if (ctx === null) return
  const board = night ? palette.canvas.dark : palette.canvas.light
  ctx.fillStyle = board
  ctx.fillRect(0, 0, width, height)
  const style = getComputedStyle(surface)
  const ink = style.color
  const family = style.getPropertyValue("--font-emoji").trim()
  const stack = family === "" ? STACK : `${family}, ${STACK}`
  const xs = Array.from({ length: across + 1 }, (_, c) => Math.round((c * width) / across))
  const ys = Array.from({ length: down + 1 }, (_, r) => Math.round((r * height) / down))
  for (let r = 0; r < down; r++) {
    for (let c = 0; c < across; c++) {
      const v = visual(fact.app, fact, fact.ids[r]?.[c] ?? 0)
      const x = xs[c] as number
      const y = ys[r] as number
      const w = (xs[c + 1] as number) - x
      const h = (ys[r + 1] as number) - y
      cell(ctx, v, fact, x, y, w, h, board, ink, stack)
    }
  }
}

function cell(
  ctx: CanvasRenderingContext2D,
  v: Visual,
  fact: Fact,
  x: number,
  y: number,
  w: number,
  h: number,
  board: string,
  ink: string,
  stack: string,
): void {
  const bg = pick(fact, v.bg)
  if (bg !== undefined) {
    const motif = v.motif === "design" ? fact.design : v.motif
    ctx.fillStyle = bg
    if (motif === undefined || !weave(ctx, motif, x, y, w, h, board)) ctx.fillRect(x, y, w, h)
  }
  const f = v.face
  if (f === undefined) return
  const tint = typeof f.tint === "object" ? (pick(fact, f.tint) ?? ink) : ink
  if (f.as === "sprite") {
    blocks(ctx, f.rows ?? [], x, y, w, h, tint)
    return
  }
  const text = f.value ?? ""
  if (text === "") return
  let size = Math.floor(h)
  ctx.font = `${size}px ${stack}`
  const wide = ctx.measureText(text).width
  if (wide > w * 0.9) {
    size = Math.max(1, Math.floor((size * w * 0.9) / wide))
    ctx.font = `${size}px ${stack}`
  }
  ctx.fillStyle = tint
  ctx.textAlign = "center"
  ctx.textBaseline = "middle"
  ctx.fillText(text, x + w / 2, y + h / 2)
}

function weave(ctx: CanvasRenderingContext2D, name: string, x: number, y: number, w: number, h: number, board: string): boolean {
  const uw = w / GRAIN
  const uh = h / GRAIN
  switch (name) {
    case "carpet": {
      ctx.fillRect(x, y, w, h)
      ctx.fillStyle = board
      for (let r = 1; r < GRAIN; r += 2) {
        for (let c = 1; c < GRAIN; c += 2) ctx.fillRect(x + c * uw, y + r * uh, uw, uh)
      }
      return true
    }
    case "net": {
      ctx.fillRect(x, y, w, h)
      ctx.fillStyle = board
      for (let r = 0; r < GRAIN; r += 2) {
        for (let c = 0; c < GRAIN; c += 2) ctx.fillRect(x + c * uw, y + r * uh, uw, uh)
      }
      return true
    }
    case "htree": {
      for (let r = 0; r < GRAIN; r += 2) ctx.fillRect(x, y + r * uh, w, uh)
      return true
    }
    case "vtree": {
      for (let c = 0; c < GRAIN; c += 2) ctx.fillRect(x + c * uw, y, uw, h)
      return true
    }
  }
  return false
}

function blocks(ctx: CanvasRenderingContext2D, rows: number[][], x: number, y: number, w: number, h: number, tint: string): void {
  const down = rows.length
  const across = rows[0]?.length ?? 0
  if (down === 0 || across === 0) return
  ctx.fillStyle = tint
  for (let r = 0; r < down; r++) {
    for (let c = 0; c < across; c++) {
      if ((rows[r]?.[c] ?? 0) === 0) continue
      const bx = x + Math.round((c * w) / across)
      const by = y + Math.round((r * h) / down)
      const bw = x + Math.round(((c + 1) * w) / across) - bx
      const bh = y + Math.round(((r + 1) * h) / down) - by
      ctx.fillRect(bx, by, bw, bh)
    }
  }
}

// CELLS

export function Cells({ app, cells, grid, crisp = true, handle, className, onTap, onDrag }: {
  app: string
  cells: Deck
  grid?: Point
  crisp?: boolean
  handle?: string
  className?: string
  onTap?: (x: number, y: number) => void
  onDrag?: (points: Point[]) => void
}) {
  const own = useRef<HTMLCanvasElement | null>(null)
  const night = dark()
  const held = useRef<{ fact: Fact; night: boolean }>({ fact: { app, ...cells }, night })
  held.current = { fact: { app, ...cells }, night }

  const paint = useCallback((surface: HTMLCanvasElement) => {
    draw(surface, held.current.fact, held.current.night)
  }, [])

  useEffect(() => {
    const el = own.current
    if (el === null || typeof ResizeObserver === "undefined") return
    const watcher = new ResizeObserver(() => draw(el, held.current.fact, held.current.night))
    watcher.observe(el)
    return () => watcher.disconnect()
  }, [])

  useEffect(() => {
    const el = own.current
    if (el === null || handle === undefined) return
    el.dataset.handle = handle
  }, [handle])

  const sig = `${night}:${app}:${cells.skin}:${cells.design ?? ""}:${cells.pens.join(",")}:${cells.ids.map(row => row.join(",")).join(";")}`
  const shape: Point = grid ?? [cells.ids[0]?.length ?? 0, cells.ids.length]

  return (
    <Canvas
      ref={own}
      grid={shape}
      paint={paint}
      sig={sig}
      crisp={crisp}
      className={className}
      onTap={onTap}
      onDrag={onDrag}
    />
  )
}
