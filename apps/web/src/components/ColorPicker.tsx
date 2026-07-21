import { h } from "../jsx.ts"
import { hex, names } from "../palette.ts"
import type { Call, Node } from "../types.ts"

type Props = { value?: string; onpick: (name: string) => Call; close: Call }

export function ColorPicker({ value, onpick, close }: Props): Node {
  return (
    <overlay key="sheet-color" close={close}>
      <card key="colors">
        <grid key="swatches" cols={5}>
          {names().map(name => (
            <button key={`c-${name}`} bg={hex(name)} call={onpick(name)}>{name === value ? "✓" : " "}</button>
          ))}
        </grid>
      </card>
    </overlay>
  )
}
