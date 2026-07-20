import { call } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = { category: string; work: string; categories: string[]; grid: string[] }

export function emoji(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="emoji">
      <card key="preview">
        <text key="current" role="title">{s.work}</text>
      </card>
      <card key="cats">
        <choice key="category" value={s.category} options={s.categories} call={call("emoji.set", { key: "category" })} arg="value" mode="row" />
      </card>
      <card key="grid">
        <grid key="emojis" cols={8}>
          {s.grid.map(e => (
            <button key={`e-${e}`} call={call("emoji.set", { key: "work", value: e })} bg={e === s.work ? "var(--accent-color)" : undefined}>{e}</button>
          ))}
        </grid>
      </card>
      <card key="free">
        <field key="entry" value={s.work} live={false} call={call("emoji.set", { key: "work" })} arg="value" label="emoji" hint="paste any" />
      </card>
    </stack>
  )
}
