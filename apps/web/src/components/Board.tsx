import { h } from "../jsx.ts"
import type { Call, Cells, Flip, Glyph, Node, Shade, Tri } from "../types.ts"

type Props = {
  app: string
  handle?: string
  keyName?: string
  rows?: number[][]
  cells?: Cells
  palette?: string[]
  glyphs?: Glyph[]
  tris?: Tri[]
  shade?: Shade
  strip?: Flip[]
  tap?: Call
  drag?: Call
  turn?: Call
  zoom?: Call
  pan?: Call
  grid?: [number, number]
}

export function Board({ app, handle, keyName = "frame", rows, cells, palette, glyphs, tris, shade, strip, tap, drag, turn, zoom, pan, grid }: Props): Node {
  return (
    <canvas
      key={keyName}
      handle={handle ?? app}
      rows={rows}
      cells={cells === undefined ? undefined : { app, ...cells }}
      palette={palette}
      glyphs={glyphs}
      tris={tris}
      shade={shade}
      strip={strip}
      tap={tap}
      drag={drag}
      turn={turn}
      zoom={zoom}
      pan={pan}
      grid={grid}
    />
  )
}
