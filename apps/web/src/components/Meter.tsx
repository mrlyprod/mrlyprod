import { h } from "../jsx.ts"
import type { Node } from "../types.ts"

export function Meter({ keyName = "status", text }: { keyName?: string; text: string }): Node {
  return (
    <card key={keyName}>
      <text key="meter" role="note">{text}</text>
    </card>
  )
}
