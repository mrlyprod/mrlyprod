import { call } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = { category: string; categories: string[]; grid: string[] }

const chunk = (items: string[], cols: number): string[][] => {
  const rows: string[][] = []
  for (let i = 0; i < items.length; i += cols) rows.push(items.slice(i, i + cols))
  return rows
}

export function emoji(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="emoji">
      <card key="cats">
        <choice key="category" value={s.category} options={s.categories} call={call("emoji.set", { key: "category" })} arg="value" mode="row" />
      </card>
      <card key="grid">
        <cells key="emojis" rows={chunk(s.grid, 8)} />
      </card>
    </stack>
  )
}
