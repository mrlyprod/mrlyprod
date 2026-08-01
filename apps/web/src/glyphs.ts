import table from "../../../files/mrlyfont/MrlyFont.json"

type Glyph = { h: number; name: string; rows: string[]; w: number }

type Revealing = HTMLElement & { __reveal?: number }

const GLYPHS = table as Record<string, Glyph>

const NS = "http://www.w3.org/2000/svg"

const REVEAL = 300

function trim(rows: string[]): string[] {
  const width = rows[0]?.length ?? 0
  const lit = (col: number) => rows.some(row => row[col] === "1")
  let start = 0
  while (start < width && !lit(start)) start++
  if (start === width) return rows.map(() => "0")
  let end = width - 1
  while (end > 0 && !lit(end)) end--
  return rows.map(row => row.slice(start, end + 1))
}

function block(text: string): { width: number; height: number; lit: [number, number][] } {
  const chars = Array.from(text)
  const tall = chars.some(c => (GLYPHS[c]?.h ?? 5) > 5)
  const height = tall ? 7 : 5
  const lit: [number, number][] = []
  let x = 0
  for (const [at, c] of chars.entries()) {
    if (at > 0) x += 1
    const glyph = c === " " ? undefined : GLYPHS[c]
    if (glyph === undefined) {
      x += 3
      continue
    }
    const rows = trim(glyph.rows)
    const drop = height === 7 && rows.length === 5 ? 1 : 0
    for (const [y, row] of rows.entries()) {
      for (let col = 0; col < row.length; col++) {
        if (row[col] === "1") lit.push([x + col, y + drop])
      }
    }
    x += rows[0]?.length ?? 0
  }
  return { width: x, height, lit }
}

function reveal(el: Revealing, rects: SVGRectElement[]) {
  for (let i = rects.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1))
    const swap = rects[i] as SVGRectElement
    rects[i] = rects[j] as SVGRectElement
    rects[j] = swap
  }
  if (el.__reveal !== undefined) cancelAnimationFrame(el.__reveal)
  const start = performance.now()
  const step = (now: number) => {
    const shown = Math.min(rects.length, Math.ceil(((now - start) / REVEAL) * rects.length))
    for (let i = 0; i < shown; i++) (rects[i] as SVGRectElement).style.fillOpacity = "1"
    el.__reveal = shown < rects.length ? requestAnimationFrame(step) : undefined
  }
  el.__reveal = requestAnimationFrame(step)
}

export function spell(el: HTMLElement, text: string): void {
  if (el.dataset.glyphs === text && el.firstElementChild instanceof SVGSVGElement) return
  el.dataset.glyphs = text
  const { width, height, lit } = block(text)
  const svg = document.createElementNS(NS, "svg")
  svg.setAttribute("class", "glyphs")
  svg.setAttribute("viewBox", `0 0 ${Math.max(width, 1)} ${height}`)
  svg.setAttribute("role", "img")
  svg.setAttribute("aria-label", text)
  const rects = lit.map(([x, y]) => {
    const rect = document.createElementNS(NS, "rect")
    rect.setAttribute("x", String(x))
    rect.setAttribute("y", String(y))
    rect.setAttribute("width", "1")
    rect.setAttribute("height", "1")
    rect.style.fillOpacity = "0"
    svg.append(rect)
    return rect
  })
  el.replaceChildren(svg)
  reveal(el as Revealing, rects)
}
