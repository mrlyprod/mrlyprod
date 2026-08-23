import { useId, useMemo, useState } from "react"
import type { ReactNode } from "react"
import { cx } from "./lib"
import { css } from "./colors"
import type { ColorName } from "./colors"

// PLOT

export type PlotScale = "linear" | "log"

export type PlotAxis = { label?: ReactNode; scale?: PlotScale; min?: number; max?: number }

export type PlotMark =
  | { kind: "curve"; f: (x: number) => number; color?: ColorName; samples?: number }
  | { kind: "seq"; terms: number[]; from?: number; color?: ColorName }
  | { kind: "points"; pts: [number, number][]; color?: ColorName }

type Span = { lo: number; hi: number; log: boolean }

type Area = { left: number; top: number; w: number; h: number }

type Tick = { p: number; text: string }

type Hit = { x: number; y: number; text: string }

type Shape =
  | { kind: "curve"; color: string; runs: string[] }
  | { kind: "seq"; color: string; stems: { x: number; base: number; tip: number }[] }
  | { kind: "points"; color: string; dots: { x: number; y: number }[] }

type Sheet = { area: Area; xt: Tick[]; yt: Tick[]; shapes: Shape[]; hits: Hit[] }

const PAD = { left: 56, right: 14, top: 14, bottom: 36 }

const ACCENT = "var(--accent-color)"

const CYCLE = [ACCENT, css("red"), css("green"), css("orange"), css("purple"), css("teal")]

const SNAP = 32

// FORMAT

function labels(vals: number[]): string[] {
  const out = vals.map(show)
  if (new Set(out).size === out.length) return out
  return vals.map(v => String(v))
}

function show(v: number): string {
  if (!Number.isFinite(v)) return "-"
  if (v === 0) return "0"
  const mag = Math.abs(v)
  if (mag >= 1e6 || mag <= 1e-4) {
    const e = Math.floor(Math.log10(mag))
    return `${String(Number((v / 10 ** e).toPrecision(4)))}e${e}`
  }
  if (Number.isInteger(v)) return String(v)
  return String(Number(v.toPrecision(6)))
}

function clamp(v: number, lo: number, hi: number): number {
  return Math.max(lo, Math.min(hi, v))
}

function tidy(v: number): number {
  if (!Number.isFinite(v)) return 0
  return Math.round(clamp(v, -1e5, 1e5) * 100) / 100
}

// SCALE

function keep(v: number, log: boolean): boolean {
  return Number.isFinite(v) && (!log || v > 0)
}

function flat(span: Span, v: number): number {
  return span.log ? Math.log10(v) : v
}

function unit(span: Span, v: number): number {
  const lo = flat(span, span.lo)
  const hi = flat(span, span.hi)
  if (hi === lo) return 0.5
  return (flat(span, v) - lo) / (hi - lo)
}

function bounds(vals: number[]): { lo: number; hi: number } | null {
  let lo = Infinity
  let hi = -Infinity
  for (const v of vals) {
    if (v < lo) lo = v
    if (v > hi) hi = v
  }
  if (!Number.isFinite(lo) || !Number.isFinite(hi)) return null
  return { lo, hi }
}

function given(v: number | undefined, log: boolean): number | null {
  if (v === undefined) return null
  return keep(v, log) ? v : null
}

function measure(axis: PlotAxis | undefined, vals: number[], log: boolean): Span {
  const low = given(axis?.min, log)
  const high = given(axis?.max, log)
  const seen = bounds(vals) ?? (log ? { lo: 1, hi: 10 } : { lo: 0, hi: 1 })
  let lo = low ?? seen.lo
  let hi = high ?? seen.hi
  if (hi < lo) {
    if (low !== null && high !== null) {
      const swap = lo
      lo = hi
      hi = swap
    } else if (low !== null) {
      hi = lo
    } else {
      lo = hi
    }
  }
  if (log) {
    const grow = 10 ** 0.05
    if (low === null) lo = lo / grow
    if (high === null) hi = hi * grow
    if (hi <= lo) {
      lo = lo / 10
      hi = hi * 10
    }
  } else {
    const gap = hi - lo
    const step = gap === 0 ? 1 : gap * 0.05
    if (low === null) lo = lo - step
    if (high === null) hi = hi + step
    if (hi <= lo) {
      lo = lo - 1
      hi = hi + 1
    }
  }
  return { lo, hi, log }
}

// TICKS

function nice(raw: number): number {
  if (!Number.isFinite(raw) || raw <= 0) return 1
  const mag = 10 ** Math.floor(Math.log10(raw))
  const norm = raw / mag
  const pick = norm < Math.SQRT2 ? 1 : norm < Math.sqrt(10) ? 2 : norm < Math.sqrt(50) ? 5 : 10
  return pick * mag
}

function ticks(span: Span): number[] {
  if (span.log) {
    const first = Math.ceil(Math.log10(span.lo) - 1e-9)
    const last = Math.floor(Math.log10(span.hi) + 1e-9)
    const decades: number[] = []
    for (let e = first; e <= last; e++) decades.push(10 ** e)
    if (decades.length === 0) return ticks({ lo: span.lo, hi: span.hi, log: false })
    const stride = Math.ceil(decades.length / 8)
    return decades.filter((_, i) => i % stride === 0)
  }
  const step = nice((span.hi - span.lo) / 6)
  const first = Math.ceil(span.lo / step) * step
  const count = Math.min(60, Math.floor((span.hi - first) / step + 1e-9))
  const dp = clamp(1 - Math.floor(Math.log10(step)), 0, 20)
  const out: number[] = []
  for (let k = 0; k <= count; k++) out.push(Number((first + k * step).toFixed(dp)))
  return out
}

// SAMPLE

function sample(f: (x: number) => number, count: number | undefined, span: Span): [number, number][] {
  const n = clamp(Math.floor(count ?? 256), 2, 4096)
  const lo = flat(span, span.lo)
  const hi = flat(span, span.hi)
  const out: [number, number][] = []
  for (let i = 0; i < n; i++) {
    const t = lo + ((hi - lo) * i) / (n - 1)
    const v = span.log ? 10 ** t : t
    out.push([v, f(v)])
  }
  return out
}

// BUILD

function within(area: Area, px: number, py: number): boolean {
  return px >= area.left - 1 && px <= area.left + area.w + 1 && py >= area.top - 1 && py <= area.top + area.h + 1
}

function build(marks: PlotMark[], x: PlotAxis | undefined, y: PlotAxis | undefined, w: number, h: number): Sheet {
  const xlog = x?.scale === "log"
  const ylog = y?.scale === "log"
  const area: Area = {
    left: PAD.left,
    top: PAD.top,
    w: Math.max(1, w - PAD.left - PAD.right),
    h: Math.max(1, h - PAD.top - PAD.bottom),
  }

  const xv: number[] = []
  for (const mark of marks) {
    if (mark.kind === "seq") {
      const from = mark.from ?? 1
      for (let k = 0; k < mark.terms.length; k++) {
        const vx = from + k
        if (keep(vx, xlog)) xv.push(vx)
      }
    }
    if (mark.kind === "points") {
      for (const [vx] of mark.pts) {
        if (keep(vx, xlog)) xv.push(vx)
      }
    }
  }
  const xspan = measure(x, xv, xlog)
  const shots: [number, number][][] = marks.map(mark =>
    mark.kind === "curve" ? sample(mark.f, mark.samples, xspan) : [],
  )

  const yv: number[] = []
  marks.forEach((mark, i) => {
    if (mark.kind === "seq") {
      const from = mark.from ?? 1
      for (const [k, term] of mark.terms.entries()) {
        if (keep(from + k, xlog) && keep(term, ylog)) yv.push(term)
      }
      if (keep(0, ylog)) yv.push(0)
    }
    if (mark.kind === "points") {
      for (const [vx, vy] of mark.pts) {
        if (keep(vx, xlog) && keep(vy, ylog)) yv.push(vy)
      }
    }
    if (mark.kind === "curve") {
      for (const [vx, vy] of shots[i] ?? []) {
        if (keep(vx, xlog) && keep(vy, ylog)) yv.push(vy)
      }
    }
  })
  const yspan = measure(y, yv, ylog)

  const atX = (v: number): number => tidy(area.left + unit(xspan, v) * area.w)
  const atY = (v: number): number => tidy(area.top + (1 - unit(yspan, v)) * area.h)
  const floor = ylog ? yspan.lo : clamp(0, yspan.lo, yspan.hi)
  const base = atY(floor)

  const shapes: Shape[] = []
  const hits: Hit[] = []
  marks.forEach((mark, i) => {
    const tint = mark.color !== undefined ? css(mark.color) : (CYCLE[i % CYCLE.length] ?? ACCENT)
    if (mark.kind === "curve") {
      const runs: string[] = []
      let run: string[] = []
      for (const [vx, vy] of shots[i] ?? []) {
        if (keep(vx, xlog) && keep(vy, ylog)) {
          const px = atX(vx)
          const py = atY(vy)
          run.push(`${px},${py}`)
          if (within(area, px, py)) hits.push({ x: px, y: py, text: `(${show(vx)}, ${show(vy)})` })
        } else {
          if (run.length > 1) runs.push(run.join(" "))
          run = []
        }
      }
      if (run.length > 1) runs.push(run.join(" "))
      shapes.push({ kind: "curve", color: tint, runs })
    }
    if (mark.kind === "seq") {
      const from = mark.from ?? 1
      const stems: { x: number; base: number; tip: number }[] = []
      for (const [k, term] of mark.terms.entries()) {
        const vx = from + k
        if (!keep(vx, xlog) || !keep(term, ylog)) continue
        const px = atX(vx)
        const py = atY(term)
        stems.push({ x: px, base, tip: py })
        if (within(area, px, py)) hits.push({ x: px, y: py, text: `a(${show(vx)}) = ${String(term)}` })
      }
      shapes.push({ kind: "seq", color: tint, stems })
    }
    if (mark.kind === "points") {
      const dots: { x: number; y: number }[] = []
      for (const [vx, vy] of mark.pts) {
        if (!keep(vx, xlog) || !keep(vy, ylog)) continue
        const px = atX(vx)
        const py = atY(vy)
        dots.push({ x: px, y: py })
        if (within(area, px, py)) hits.push({ x: px, y: py, text: `(${show(vx)}, ${show(vy)})` })
      }
      shapes.push({ kind: "points", color: tint, dots })
    }
  })

  const xs = ticks(xspan)
  const ys = ticks(yspan)
  const xl = labels(xs)
  const yl = labels(ys)
  return {
    area,
    xt: xs.map((v, i) => ({ p: atX(v), text: xl[i] ?? show(v) })),
    yt: ys.map((v, i) => ({ p: atY(v), text: yl[i] ?? show(v) })),
    shapes,
    hits,
  }
}

// HOVER

function same(a: Hit | null, b: Hit | null): boolean {
  if (a === null || b === null) return a === b
  return a.x === b.x && a.y === b.y && a.text === b.text
}

function place(hit: Hit, w: number, h: number): { left: string; top: string; transform: string } {
  const dx = hit.x > w * 0.6 ? "calc(-100% - 10px)" : "10px"
  const dy = hit.y < h * 0.3 ? "10px" : "calc(-100% - 10px)"
  return {
    left: `${clamp((hit.x / w) * 100, 1, 99)}%`,
    top: `${clamp((hit.y / h) * 100, 1, 99)}%`,
    transform: `translate(${dx}, ${dy})`,
  }
}

// FIGURE

export function Plot({ marks, x, y, width, height, className }: {
  marks: PlotMark[]
  x?: PlotAxis
  y?: PlotAxis
  width?: number
  height?: number
  className?: string
}) {
  const w = width ?? 640
  const h = height ?? 420
  const uid = useId().replace(/:/g, "")
  const [hover, setHover] = useState<Hit | null>(null)
  const sheet = useMemo(
    () => build(marks, x, y, w, h),
    [marks, x?.scale, x?.min, x?.max, y?.scale, y?.min, y?.max, w, h],
  )
  const area = sheet.area
  const xlabel = x?.label ?? null
  const ylabel = y?.label ?? null

  const probe = (box: DOMRect, clientX: number, clientY: number): void => {
    if (box.width === 0 || box.height === 0) return
    const px = ((clientX - box.left) / box.width) * w
    const py = ((clientY - box.top) / box.height) * h
    let near = SNAP * SNAP
    let found: Hit | null = null
    for (const hit of sheet.hits) {
      const gap = (hit.x - px) ** 2 + (hit.y - py) ** 2
      if (gap <= near) {
        near = gap
        found = hit
      }
    }
    setHover(held => (same(held, found) ? held : found))
  }

  return (
    <figure className={cx("plot", className)}>
      {ylabel !== null && <div className="plot-y">{ylabel}</div>}
      <div className="plot-body">
        <svg
          viewBox={`0 0 ${w} ${h}`}
          onPointerMove={event => probe(event.currentTarget.getBoundingClientRect(), event.clientX, event.clientY)}
          onPointerLeave={() => setHover(null)}
        >
          <defs>
            <clipPath id={`plot-${uid}`}>
              <rect x={area.left} y={area.top} width={area.w} height={area.h} />
            </clipPath>
          </defs>
          <g stroke="var(--hairline-color)" strokeWidth={1}>
            {sheet.xt.map((tick, i) => (
              <line key={`gx${i}`} x1={tick.p} y1={area.top} x2={tick.p} y2={area.top + area.h} />
            ))}
            {sheet.yt.map((tick, i) => (
              <line key={`gy${i}`} x1={area.left} y1={tick.p} x2={area.left + area.w} y2={tick.p} />
            ))}
          </g>
          <g stroke="var(--font-color)" strokeWidth={1.5}>
            <line x1={area.left} y1={area.top} x2={area.left} y2={area.top + area.h} />
            <line x1={area.left} y1={area.top + area.h} x2={area.left + area.w} y2={area.top + area.h} />
          </g>
          <g clipPath={`url(#plot-${uid})`}>
            {sheet.shapes.map((shape, i) => {
              if (shape.kind === "curve") {
                return (
                  <g
                    key={i}
                    fill="none"
                    stroke={shape.color}
                    strokeWidth={2}
                    strokeLinecap="round"
                    strokeLinejoin="round"
                  >
                    {shape.runs.map((run, k) => (
                      <polyline key={`r${k}`} points={run} />
                    ))}
                  </g>
                )
              }
              if (shape.kind === "seq") {
                return (
                  <g key={i} fill={shape.color} stroke={shape.color}>
                    {shape.stems.map((stem, k) => (
                      <line
                        key={`s${k}`}
                        x1={stem.x}
                        y1={stem.base}
                        x2={stem.x}
                        y2={stem.tip}
                        strokeWidth={1.5}
                        opacity={0.45}
                      />
                    ))}
                    {shape.stems.map((stem, k) => (
                      <circle key={`d${k}`} cx={stem.x} cy={stem.tip} r={3.5} stroke="none" />
                    ))}
                  </g>
                )
              }
              return (
                <g key={i} fill={shape.color}>
                  {shape.dots.map((dot, k) => (
                    <circle key={`p${k}`} cx={dot.x} cy={dot.y} r={3} />
                  ))}
                </g>
              )
            })}
          </g>
          <g fill="var(--muted-color)" fontSize={12}>
            {sheet.xt.map((tick, i) => (
              <text key={`lx${i}`} x={tick.p} y={area.top + area.h + 20} textAnchor="middle">
                {tick.text}
              </text>
            ))}
            {sheet.yt.map((tick, i) => (
              <text key={`ly${i}`} x={area.left - 10} y={tick.p + 4} textAnchor="end">
                {tick.text}
              </text>
            ))}
          </g>
          {hover !== null && (
            <circle cx={hover.x} cy={hover.y} r={6} fill="none" stroke="var(--accent-color)" strokeWidth={1.5} />
          )}
        </svg>
        {hover !== null && (
          <div className="plot-read" style={place(hover, w, h)}>
            {hover.text}
          </div>
        )}
      </div>
      {xlabel !== null && <div className="plot-x">{xlabel}</div>}
    </figure>
  )
}
