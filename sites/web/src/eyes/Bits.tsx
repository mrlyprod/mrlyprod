import { useCallback, useEffect, useRef } from "react"
import { Canvas } from "mrlyui"
import type { Point } from "mrlyui"
import { dark } from "./theme"

// FILL

function fill(surface: HTMLCanvasElement, rows: number[][], pens: string[] | undefined): void {
  const height = rows.length
  const width = rows[0]?.length ?? 0
  if (surface.width !== width) surface.width = width
  if (surface.height !== height) surface.height = height
  const ctx = surface.getContext("2d")
  if (ctx === null || width === 0) return
  ctx.clearRect(0, 0, width, height)
  if (pens !== undefined) {
    for (let y = 0; y < height; y++) {
      const row = rows[y] as number[]
      for (let x = 0; x < width; x++) {
        ctx.fillStyle = pens[row[x] as number] ?? "#000000"
        ctx.fillRect(x, y, 1, 1)
      }
    }
    return
  }
  ctx.fillStyle = getComputedStyle(surface).color
  for (let y = 0; y < height; y++) {
    const row = rows[y] as number[]
    for (let x = 0; x < width; x++) {
      const cell = row[x] as number
      if (cell === 0) continue
      ctx.globalAlpha = cell / 255
      ctx.fillRect(x, y, 1, 1)
    }
  }
  ctx.globalAlpha = 1
}

// BITS

export function Bits({ rows, palette, grid, crisp, handle, className, onTap, onDrag }: {
  rows: number[][]
  palette?: string[]
  grid?: Point
  crisp?: boolean
  handle?: string
  className?: string
  onTap?: (x: number, y: number) => void
  onDrag?: (points: Point[]) => void
}) {
  const own = useRef<HTMLCanvasElement | null>(null)
  const held = useRef<{ rows: number[][]; palette: string[] | undefined }>({ rows, palette })
  held.current = { rows, palette }

  const paint = useCallback((surface: HTMLCanvasElement) => {
    fill(surface, held.current.rows, held.current.palette)
  }, [])

  useEffect(() => {
    const el = own.current
    if (el === null || handle === undefined) return
    el.dataset.handle = handle
  }, [handle])

  const sig = `${dark()}:${palette?.join(",") ?? ""}:${rows.map(row => row.join(",")).join(";")}`
  const shape: Point = grid ?? [rows[0]?.length ?? 0, rows.length]

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
