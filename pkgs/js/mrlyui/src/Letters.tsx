import { useEffect, useMemo, useState } from "react"
import { cx } from "./lib"
import font from "./gen/mrlyfont.json"

// FONT

type Char = {
  name: string
  w: number
  h: number
  rows: string[]
}

const FONT = font as Record<string, Char>

const GAP = 1

const BODY = 5

const FALLBACK: Char = { name: "FALLBACK", w: 5, h: 5, rows: ["11111", "10001", "10101", "10001", "11111"] }

function glyph(char: string): Char {
  return FONT[char] ?? FONT["?"] ?? FALLBACK
}

// REVEAL

type Dot = { x: number; y: number }

function Reveal({ pixels, box, scramble, pace, className, label }: {
  pixels: Dot[]
  box: string
  scramble: boolean
  pace: number
  className?: string
  label?: string
}) {
  const order = useMemo(() => {
    const indices = pixels.map((_, i) => i)
    for (let i = indices.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1))
      const a = indices[i]
      const b = indices[j]
      if (a !== undefined && b !== undefined) {
        indices[i] = b
        indices[j] = a
      }
    }
    return indices
  }, [pixels])

  const [shown, setShown] = useState(() => (scramble ? 0 : pixels.length))

  useEffect(() => {
    if (!scramble) {
      setShown(pixels.length)
      return
    }
    setShown(0)
    if (pixels.length === 0) return
    const step = Math.max(10, pace / pixels.length)
    let current = 0
    const interval = setInterval(() => {
      current++
      setShown(current)
      if (current >= pixels.length) clearInterval(interval)
    }, step)
    return () => clearInterval(interval)
  }, [scramble, pace, pixels])

  return (
    <svg
      className={cx("letters", className)}
      viewBox={box}
      role={label ? "img" : undefined}
      aria-label={label || undefined}
      aria-hidden={label ? undefined : true}
    >
      {order.slice(0, shown).map(i => {
        const pixel = pixels[i]
        if (!pixel) return null
        return <rect key={i} x={pixel.x} y={pixel.y} width={1} height={1} />
      })}
    </svg>
  )
}

// PIXELS

export function Pixels({ grid, scramble = true, pace = 300, className, label }: {
  grid: string[] | number[][]
  scramble?: boolean
  pace?: number
  className?: string
  label?: string
}) {
  const { pixels, width } = useMemo(() => {
    const dots: Dot[] = []
    let width = 0
    grid.forEach((row, y) => {
      const cells = typeof row === "string" ? [...row].map(c => (c === "1" ? 1 : 0)) : row
      width = Math.max(width, cells.length)
      cells.forEach((v, x) => {
        if (v) dots.push({ x, y })
      })
    })
    return { pixels: dots, width }
  }, [grid])
  return (
    <Reveal
      pixels={pixels}
      box={`0 0 ${width} ${grid.length}`}
      scramble={scramble}
      pace={pace}
      className={className}
      label={label}
    />
  )
}

// LETTERS

export function letters(char: string): string[] {
  return glyph(char).rows
}

export function Letters({ text, scramble = true, pace = 300, className, label = text }: {
  text: string
  scramble?: boolean
  pace?: number
  className?: string
  label?: string
}) {
  const { pixels, width, minY, height } = useMemo(() => {
    const chars = [...text].map(glyph)
    const pixels: Dot[] = []
    let cursor = 0
    let top = 0
    let bottom = BODY
    for (const char of chars) {
      const offset = Math.floor((BODY - char.h) / 2)
      top = Math.min(top, offset)
      bottom = Math.max(bottom, offset + char.h)
      char.rows.forEach((row, r) => {
        for (let c = 0; c < char.w; c++) {
          if (row[c] === "1") pixels.push({ x: cursor + c, y: offset + r })
        }
      })
      cursor += char.w + GAP
    }
    return {
      pixels,
      width: chars.length > 0 ? cursor - GAP : 0,
      minY: top,
      height: bottom - top,
    }
  }, [text])

  return (
    <Reveal
      pixels={pixels}
      box={`0 ${minY} ${width} ${height}`}
      scramble={scramble}
      pace={pace}
      className={className}
      label={label}
    />
  )
}
