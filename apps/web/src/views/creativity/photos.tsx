import { call } from "../../builders.ts"
import { Meter } from "../../components/Meter.tsx"
import { h } from "../../jsx.ts"
import type { Flip, Node, Send } from "../../types.ts"

type State = {
  shots: number
  photos: Flip[]
}

export function photos(state: unknown, _send: Send): Node {
  const s = state as State
  const meter = s.photos.length === 0 ? "an empty wall" : `${s.photos.length} on the wall`
  return (
    <stack key="photos">
      <card key="controls">
        <button key="clear" call={call("photos.clear")}>clear</button>
      </card>
      <card key="wall">
        <grid key="wall" cols={3}>
          {s.photos.map((p, i) => (
            <canvas key={`p-${i}`} handle={`photo-${i}`} rows={p.rows} palette={p.palette} />
          ))}
        </grid>
      </card>
      <Meter keyName="meter" text={meter} />
    </stack>
  )
}
