const EMOJI = ["🌸", "⭐", "🌙", "🍀", "🌈", "☁️", "🦋", "🍉", "🐚", "🌵", "🍄", "❄️", "🎈", "🐟", "🌻", "🪐"]

let source = "color"
let seed = 0

function rng(start: number): () => number {
  let s = Math.floor(start) + 1
  return () => {
    s = (s * 48271) % 2147483647
    return s / 2147483647
  }
}

function draw(): void {
  if (source !== "pattern") {
    document.body.style.removeProperty("background-image")
    return
  }
  const next = rng(seed)
  const emoji = EMOJI[Math.floor(next() * EMOJI.length)] as string
  const size = 48 + Math.floor(next() * 5) * 24
  const angle = Math.floor(next() * 9) * 10 - 40
  const accent = getComputedStyle(document.documentElement).getPropertyValue("--accent-color").trim()
  const font = Math.round(size * 0.32)
  const quarter = size / 4
  const glyph = (x: number, y: number) =>
    `<text x="${x}" y="${y}" font-size="${font}" text-anchor="middle" dominant-baseline="central" opacity="0.5" transform="rotate(${angle} ${x} ${y})">${emoji}</text>`
  const svg =
    `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}">` +
    `<rect width="${size}" height="${size}" fill="${accent}" opacity="0.08"/>` +
    glyph(quarter, quarter) +
    glyph(quarter * 3, quarter * 3) +
    `</svg>`
  document.body.style.backgroundImage = `url("data:image/svg+xml,${encodeURIComponent(svg)}")`
}

export function setSource(value: string): void {
  source = value
  draw()
}

export function setSeed(value: number): void {
  seed = value
  draw()
}

export function refresh(): void {
  draw()
}
