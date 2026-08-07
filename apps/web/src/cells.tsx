import skinsData from "./gen/skins.json"
import { h } from "./jsx.ts"
import type { Cells, Node, Skins, Visual } from "./types.ts"

const skins = skinsData as Skins

export function visual(app: string, cells: Cells, id: number): Visual {
  return skins[app]?.[cells.skin]?.[id] ?? {}
}

export function paint(v: Visual, cells: Cells): string | undefined {
  if (v.bg === undefined) return undefined
  return typeof v.bg === "string" ? v.bg : cells.pens[v.bg.pen]
}

export function face(v: Visual, key = "face"): Node | undefined {
  const f = v.face
  if (f === undefined || f.as === "sprite" || f.value === undefined) return undefined
  return <symbol key={key} as={f.as} value={f.value} />
}
