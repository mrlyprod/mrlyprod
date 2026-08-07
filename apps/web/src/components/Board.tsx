import { h } from "../jsx.ts"
import type { Call, Cells, Node, Shade } from "../types.ts"

type Props = {
  app: string
  handle?: string
  keyName?: string
  cells?: Cells
  tris?: number[]
  shade?: Shade
  tap?: Call
  drag?: Call
  turn?: Call
  zoom?: Call
  pan?: Call
  grid?: [number, number]
}

export function Board({ app, handle, keyName = "frame", cells, tris, shade, tap, drag, turn, zoom, pan, grid }: Props): Node {
  return (
    <canvas
      key={keyName}
      handle={handle ?? app}
      cells={cells === undefined ? undefined : { app, ...cells }}
      tris={tris}
      shade={shade}
      tap={tap}
      drag={drag}
      turn={turn}
      zoom={zoom}
      pan={pan}
      grid={grid}
    />
  )
}
