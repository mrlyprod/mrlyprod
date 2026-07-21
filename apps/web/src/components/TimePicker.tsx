import { h } from "../jsx.ts"
import type { Call, Node } from "../types.ts"

type Props = {
  h: number
  m: number
  turn: (key: "h" | "m", value: number) => Call
  onpick: (v: { h: number; m: number }) => Call
  close: Call
}

const pad = (n: number) => String(n).padStart(2, "0")

export function TimePicker({ h: hh, m: mm, turn, onpick, close }: Props): Node {
  return (
    <overlay key="sheet-time" close={close}>
      <card key="time">
        <text key="face" role="title">{`${pad(hh)}:${pad(mm)}`}</text>
        <grid key="steppers" cols={2}>
          <button key="h-dec" call={turn("h", Math.max(0, hh - 1))}>- hr</button>
          <button key="h-inc" call={turn("h", Math.min(23, hh + 1))}>+ hr</button>
          <button key="m-dec" call={turn("m", Math.max(0, mm - 1))}>- min</button>
          <button key="m-inc" call={turn("m", Math.min(59, mm + 1))}>+ min</button>
        </grid>
        <button key="ok" call={onpick({ h: hh, m: mm })}>ok</button>
      </card>
    </overlay>
  )
}
