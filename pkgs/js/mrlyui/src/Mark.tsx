import { useEffect, useRef } from "react"
import { cx } from "./lib"
import { POOL } from "./colors"
import type { ColorName } from "./colors"
import markData from "./gen/mark.json"

type Face = {
  cols: number
  rows: number
  fps: number
  frames: number[][]
}

const FACE = markData as Face

function roll(): ColorName {
  return POOL[Math.floor(Math.random() * POOL.length)] ?? "blue"
}

export function Mark({ className }: { className?: string }) {
  const frame = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    const el = frame.current
    if (el === null) return
    const ctx = el.getContext("2d")
    if (ctx === null || FACE.frames.length === 0) return
    const held = new Map<number, ColorName>()
    let at = 0
    let seen = true
    const eye = new IntersectionObserver(entries => {
      seen = entries[0]?.isIntersecting ?? true
    })
    eye.observe(el)
    const timer = setInterval(() => {
      if (!seen) return
      const lit = FACE.frames[at % FACE.frames.length] ?? []
      at += 1
      ctx.clearRect(0, 0, FACE.cols, FACE.rows)
      const style = getComputedStyle(el)
      const on = new Set(lit)
      for (const idx of held.keys()) if (!on.has(idx)) held.delete(idx)
      for (const idx of lit) {
        const hue = held.get(idx) ?? roll()
        held.set(idx, hue)
        ctx.fillStyle = style.getPropertyValue(`--c-${hue}`)
        ctx.fillRect(idx % FACE.cols, Math.floor(idx / FACE.cols), 1, 1)
      }
    }, 1000 / FACE.fps)
    return () => {
      clearInterval(timer)
      eye.disconnect()
    }
  }, [])

  return (
    <canvas
      ref={frame}
      className={cx("mark", className)}
      width={FACE.cols}
      height={FACE.rows}
      aria-label="mrlyprod"
      role="img"
    />
  )
}
