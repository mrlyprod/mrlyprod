import { call } from "../builders.ts"
import type { Node } from "../types.ts"

export function Shot(): Node {
  return { kind: "Button", key: "shot", label: "screenshot", call: call("sys.shot") }
}
