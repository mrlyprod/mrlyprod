import { Emoji, Letters } from "mrlyui"
import type { Visual } from "../types"

export function Face({ visual }: { visual: Visual }) {
  const f = visual.face
  if (f === undefined || f.as === "sprite" || f.value === undefined || f.value === "") return null
  if (f.as === "emoji") return <Emoji glyph={f.value} />
  return <Letters text={f.value} />
}
