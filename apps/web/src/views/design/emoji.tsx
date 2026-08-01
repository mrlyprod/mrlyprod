import { call } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = { category: string; categories: string[]; grid: string[]; library: string[] }

export function emoji(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="emoji">
      <card key="cats">
        <choice key="category" value={s.category} options={s.categories} call={call("emoji.set", { key: "category" })} arg="value" mode="row" />
      </card>
      <card key="grid">
        <grid key="emojis" cols={8}>
          {s.grid.map(value => (
            <button key={`e-${value}`} call={call("emoji.keep", { value })}>{value}</button>
          ))}
        </grid>
      </card>
      <card key="library">
        <text key="drop-hint" role="note">tap to drop</text>
        <grid key="lib" cols={8}>
          {s.library.map(value => (
            <button key={`lib-${value}`} call={call("emoji.drop", { value })}>{value}</button>
          ))}
        </grid>
      </card>
    </stack>
  )
}
