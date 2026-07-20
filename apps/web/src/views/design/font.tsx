import { call, raster } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Raster, Send } from "../../types.ts"

type State = { char: string; name: string; index: number; total: number; revealing: boolean; glyph: Raster }

const FORMATS = ["json", "ttf", "woff", "woff2"]

export function font(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="font">
      <card key="detail">
        {raster("glyph", "font", s.glyph)}
        <text key="char" role="title">{s.char}</text>
        <text key="facts" role="note">{`${s.name} · ${s.glyph.width}x${s.glyph.height} · ${s.index + 1}/${s.total}`}</text>
      </card>
      <card key="browse">
        <button key="prev" call={call("font.prev")}>←</button>
        <button key="next" call={call("font.next")}>→</button>
        <field key="pick" value="" live={false} call={call("font.pick")} arg="char" hint="char" />
        <button key="scramble" call={call("font.scramble")}>scramble</button>
      </card>
      <card key="export">
        {FORMATS.map(f => (
          <button key={f} call={call("font.export", { format: f })}>{f}</button>
        ))}
      </card>
    </stack>
  )
}
