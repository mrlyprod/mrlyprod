import { h } from "../jsx.ts"
import type { Call, Flip, Node, Shade } from "../types.ts"

type Props = {
  app: string
  handle?: string
  keyName?: string
  rows: number[][]
  palette?: string[]
  shade?: Shade
  strip?: Flip[]
  tap?: Call
  drag?: Call
  turn?: Call
  zoom?: Call
  pan?: Call
  grid?: [number, number]
}

export function Board({ app, handle, keyName = "frame", rows, palette, shade, strip, tap, drag, turn, zoom, pan, grid }: Props): Node {
  return (
    <canvas
      key={keyName}
      handle={handle ?? app}
      rows={rows}
      palette={palette}
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
