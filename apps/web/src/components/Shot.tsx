import { call } from "../builders.ts"
import type { Call, Node } from "../types.ts"

const SIDE = 320

export function Shot(): Node {
  return { kind: "Button", key: "shot", label: "screenshot", call: call("sys.shot") }
}

export function shoot(region: HTMLElement, made: Call): Call {
  if (made.args["image"] !== undefined) return made
  const surface = region.querySelector("canvas")
  if (surface === null || surface.width === 0 || surface.height === 0) return made
  const image = photo(surface)
  return image === null ? made : { verb: made.verb, args: { image } }
}

function pixels(surface: HTMLCanvasElement): ImageData | null {
  const direct = surface.getContext("2d")
  if (direct !== null) return direct.getImageData(0, 0, surface.width, surface.height)
  const copy = document.createElement("canvas")
  copy.width = surface.width
  copy.height = surface.height
  const ctx = copy.getContext("2d")
  if (ctx === null) return null
  ctx.drawImage(surface, 0, 0)
  return ctx.getImageData(0, 0, copy.width, copy.height)
}

function hex(data: Uint8ClampedArray, at: number): string {
  const byte = (v: number) => v.toString(16).padStart(2, "0")
  const shade = `#${byte(data[at] as number)}${byte(data[at + 1] as number)}${byte(data[at + 2] as number)}`
  const alpha = data[at + 3] as number
  return alpha === 255 ? shade : `${shade}${byte(alpha)}`
}

function photo(surface: HTMLCanvasElement): Record<string, unknown> | null {
  let read: ImageData | null = null
  try {
    read = pixels(surface)
  } catch {
    return null
  }
  if (read === null) return null
  const scale = Math.min(1, SIDE / Math.max(read.width, read.height))
  const width = Math.max(1, Math.round(read.width * scale))
  const height = Math.max(1, Math.round(read.height * scale))
  const palette: string[] = []
  const seen = new Map<string, number>()
  const rows: number[][] = []
  for (let y = 0; y < height; y++) {
    const sy = Math.min(read.height - 1, Math.floor(((y + 0.5) * read.height) / height))
    const row: number[] = []
    for (let x = 0; x < width; x++) {
      const sx = Math.min(read.width - 1, Math.floor(((x + 0.5) * read.width) / width))
      const shade = hex(read.data, (sy * read.width + sx) * 4)
      let id = seen.get(shade)
      if (id === undefined) {
        id = palette.length
        seen.set(shade, id)
        palette.push(shade)
      }
      row.push(id)
    }
    rows.push(row)
  }
  return { width, height, rows, palette }
}
