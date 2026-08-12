import skinsData from "../gen/skins.json"
import type { Cells, Skins, Visual } from "../types"

const skins = skinsData as Skins

export function visual(app: string, cells: Cells, id: number): Visual {
  return skins[app]?.[cells.skin]?.[id] ?? {}
}

export function paint(v: Visual, cells: Cells): string | undefined {
  if (v.bg === undefined) return undefined
  return typeof v.bg === "string" ? v.bg : cells.pens[v.bg.pen]
}
