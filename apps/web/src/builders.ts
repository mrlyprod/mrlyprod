import type { Args, Call, Node, Raster } from "./types.ts"

export const call = (verb: string, args: Args = {}): Call => ({ verb, args })

export const set = (app: string, key: string): Call => ({ verb: `${app}.set`, args: { key } })

export const setter = (app: string) => (key: string): Call => set(app, key)

export const pickColor = (host: string, key: string, value?: string): Call =>
  call("sheet.open", value === undefined ? { picker: "color", host, key } : { picker: "color", host, key, value })

export const pickGlyph = (host: string, key: string, set: string, value?: string): Call =>
  call("sheet.open", value === undefined ? { picker: "glyph", host, key, set } : { picker: "glyph", host, key, set, value })

export const pickTime = (host: string, key: string, value: { h: number; m: number }): Call =>
  call("sheet.open", { picker: "time", host, key, value })

export const pickTile = (host: string, key: string): Call =>
  call("sheet.open", { picker: "tile", host, key })

export const raster = (key: string, handle: string, glyph: Raster): Node => ({
  kind: "Canvas",
  key,
  handle,
  rows: glyph.rows.map(row => row.map(cell => (cell === 0 ? 0 : 255))),
})
