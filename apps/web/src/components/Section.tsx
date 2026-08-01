import { h } from "../jsx.ts"
import type { Kids } from "../jsx.ts"
import type { Node } from "../types.ts"

export function Section({ keyName, label, children }: { keyName: string; label: string; children?: Kids }): Node {
  return (
    <card key={keyName}>
      <text key={`${keyName}-label`} role="label">{label}</text>
      {children}
    </card>
  )
}
