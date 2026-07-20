import { call } from "../../builders.ts"
import { Meter } from "../../components/Meter.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  shots: number
  waiting: number
  photos: string[]
}

export function photos(state: unknown, _send: Send): Node {
  const s = state as State
  const meter =
    s.waiting > 0 ? `fetching ${s.waiting}…` : s.photos.length === 0 ? "an empty wall" : `${s.photos.length} on the wall`
  return (
    <stack key="photos">
      <card key="controls">
        <grid key="controls" cols={2}>
          <button key="load" call={call("photos.load")}>load</button>
          <button key="clear" call={call("photos.clear")}>clear</button>
        </grid>
      </card>
      <card key="wall">
        <grid key="wall" cols={3}>
          {s.photos.map((src, i) => (
            <stack key={`frame-${i}`}>
              <image key={`p-${i}`} src={src} alt={`photo ${i + 1}`} />
              <label key={`save-${i}`} mode="text" text={`mrly-${i}.png`} href={src} />
            </stack>
          ))}
        </grid>
      </card>
      <Meter keyName="meter" text={meter} />
    </stack>
  )
}
