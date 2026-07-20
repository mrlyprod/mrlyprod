import { call } from "../builders.ts"
import type { Node } from "../types.ts"

export function GameOver({ app, emoji, status }: { app: string; emoji: string; status: string }): Node {
  return {
    kind: "Card",
    key: "over",
    children: [
      { kind: "Symbol", key: "over-emoji", as: "emoji", value: emoji },
      { kind: "Text", key: "over-title", text: "game over", role: "title" },
      { kind: "Text", key: "over-status", text: status, role: "note" },
      { kind: "Button", key: "again", label: "play again", call: call(`${app}.reset`) },
    ],
  }
}
