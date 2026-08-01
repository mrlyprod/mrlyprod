import { h } from "./jsx.ts"
import type { Node } from "./types.ts"

export type Face = { as: "glyph" | "emoji" | "sprite"; value?: string; rows?: number[][] }

export type Visual = { bg?: string; motif?: string; face?: Face }

export type Skin = Visual[]

export function visual(skin: Skin | undefined, role: number): Visual {
  return skin?.[role] ?? {}
}

export function face(v: Visual, key = "face"): Node | undefined {
  const f = v.face
  if (!f || f.as === "sprite" || !f.value) return undefined
  return <symbol key={key} as={f.as} value={f.value} />
}
