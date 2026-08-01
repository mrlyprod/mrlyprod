import { call } from "../builders.ts"
import { colorpicker } from "./colorpicker.tsx"
import { hex, names } from "../palette.ts"
import { peeked } from "../peeks.ts"
import type { Node } from "../types.ts"

export function palette(app: string, colors: string[]): Node[] {
  const lib = (peeked("colors")?.state as { library?: string[] } | undefined)?.library ?? []
  const pool = lib.length > 0 ? lib : names()
  const picked = new Set(colors.map(c => c.toLowerCase()))
  const swatches = pool.map(name => ({ name, hex: hex(name) }))
  const on = (name: string) => picked.has(hex(name).toLowerCase())
  const pick = (name: string) => {
    const swatch = hex(name)
    const next = on(name) ? colors.filter(c => c.toLowerCase() !== swatch.toLowerCase()) : [...colors, swatch]
    return call(`${app}.set`, { key: "palette", value: next })
  }
  return [colorpicker("palette", swatches, on, pick)]
}
