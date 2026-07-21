import { call, pickColor } from "../builders.ts"
import { h } from "../jsx.ts"
import type { Node } from "../types.ts"

export function palette(app: string, host: string, colors: string[]): Node[] {
  return [
    ...colors.map((hex, i) => (
      <button key={`slot-${i}`} bg={hex} call={pickColor(host, `palette.${i}`)}>{" "}</button>
    )),
    ...(colors.length > 1
      ? colors.map((_, i) => (
          <button key={`drop-${i}`} call={call(`${app}.set`, { key: "palette", value: colors.filter((_, j) => j !== i) })}>{`× ${i + 1}`}</button>
        ))
      : []),
    <button key="add" call={call(`${app}.set`, { key: "palette", value: [...colors, colors[colors.length - 1] ?? "#ffffff"] })}>+</button>,
  ]
}
