import { h } from "../jsx.ts"
import type { Call, GlyphSet, Node } from "../types.ts"

type Props = {
  sets: GlyphSet[]
  cat: number
  value?: string
  onpick: (glyph: string) => Call
  turn: (cat: number) => Call
  close: Call
}

export function GlyphPicker({ sets, cat, value, onpick, turn, close }: Props): Node {
  const active = sets[cat] ?? sets[0]
  const glyphs = active?.glyphs ?? []
  return (
    <overlay key="sheet-glyph" close={close}>
      <card key="glyphs">
        {sets.length > 1 && (
          <grid key="tabs" cols={3}>
            {sets.map((s, i) => (
              <button key={`t-${i}`} call={turn(i)} bg={i === cat ? "var(--accent-color)" : undefined}>{s.name}</button>
            ))}
          </grid>
        )}
        <grid key="grid" cols={8}>
          {glyphs.map(g => (
            <button key={`g-${g}`} call={onpick(g)} bg={g === value ? "var(--accent-color)" : undefined}>{g}</button>
          ))}
        </grid>
      </card>
    </overlay>
  )
}
