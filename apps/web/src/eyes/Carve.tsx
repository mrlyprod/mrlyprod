import { useCallback, useEffect, useRef } from "react"
import { Canvas } from "mrlyui"

const STRIDE = 10

// DRAW

function carve(surface: HTMLCanvasElement, buf: number[]): void {
  if (buf.length <= 1) return
  const rect = surface.getBoundingClientRect()
  if (rect.width === 0 || rect.height === 0) return
  const dpr = globalThis.devicePixelRatio || 1
  const width = Math.max(1, Math.round(rect.width * dpr))
  const height = Math.max(1, Math.round(rect.height * dpr))
  if (surface.width !== width) surface.width = width
  if (surface.height !== height) surface.height = height
  const ctx = surface.getContext("2d")
  if (ctx === null) return
  ctx.clearRect(0, 0, width, height)
  let minX = Infinity
  let maxX = -Infinity
  let minY = Infinity
  let maxY = -Infinity
  for (let i = 1; i + STRIDE <= buf.length; i += STRIDE) {
    for (let p = 0; p < 3; p++) {
      const x = buf[i + 2 * p] as number
      const y = buf[i + 2 * p + 1] as number
      minX = Math.min(minX, x)
      maxX = Math.max(maxX, x)
      minY = Math.min(minY, y)
      maxY = Math.max(maxY, y)
    }
  }
  const span = Math.max(maxX - minX, maxY - minY, 1)
  const scale = (Math.min(width, height) * 0.9) / span
  const cx = (minX + maxX) / 2
  const cy = (minY + maxY) / 2
  for (let i = 1; i + STRIDE <= buf.length; i += STRIDE) {
    const r = Math.round((buf[i + 6] as number) * 255)
    const g = Math.round((buf[i + 7] as number) * 255)
    const b = Math.round((buf[i + 8] as number) * 255)
    const a = buf[i + 9] as number
    ctx.fillStyle = `rgba(${r}, ${g}, ${b}, ${a})`
    ctx.beginPath()
    for (let p = 0; p < 3; p++) {
      const px = width / 2 + ((buf[i + 2 * p] as number) - cx) * scale
      const py = height / 2 + ((buf[i + 2 * p + 1] as number) - cy) * scale
      if (p === 0) ctx.moveTo(px, py)
      else ctx.lineTo(px, py)
    }
    ctx.closePath()
    ctx.fill()
  }
}

// CARVE

export function Carve({ tris, crisp = true, handle, className }: {
  tris: number[]
  crisp?: boolean
  handle?: string
  className?: string
}) {
  const own = useRef<HTMLCanvasElement | null>(null)
  const held = useRef(tris)
  held.current = tris

  const paint = useCallback((surface: HTMLCanvasElement) => {
    carve(surface, held.current)
  }, [])

  useEffect(() => {
    const el = own.current
    if (el === null || typeof ResizeObserver === "undefined") return
    const watcher = new ResizeObserver(() => carve(el, held.current))
    watcher.observe(el)
    return () => watcher.disconnect()
  }, [])

  useEffect(() => {
    const el = own.current
    if (el === null || handle === undefined) return
    el.dataset.handle = handle
  }, [handle])

  return <Canvas ref={own} paint={paint} sig={tris.join(",")} crisp={crisp} className={className} />
}
