import type { Args, Call, Node, Raster } from "./types.ts"

export const call = (verb: string, args: Args = {}): Call => ({ verb, args })

export const set = (app: string, key: string): Call => ({ verb: `${app}.set`, args: { key } })

export const setter = (app: string) => (key: string): Call => set(app, key)

export const raster = (key: string, handle: string, glyph: Raster): Node => ({
  kind: "Canvas",
  key,
  handle,
  rows: glyph.rows.map(row => row.map(cell => (cell === 0 ? 0 : 255))),
})
