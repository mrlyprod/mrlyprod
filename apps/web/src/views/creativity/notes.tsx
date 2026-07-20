import { call } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = { query: string; found: { id: number; text: string }[] }

export function notes(state: unknown, _send: Send): Node {
  const { query, found } = state as State
  const noteCards = found.map(item => {
    const key = `note-${item.id}`
    return (
      <card key={key}>
        <field key={`${key}-text`} value={item.text} live={false} call={call("notes.edit", { id: item.id })} arg="text" />
        <button key={`${key}-remove`} call={call("notes.remove", { id: item.id })}>✕</button>
      </card>
    )
  })
  return (
    <stack key="notes">
      <card key="compose">
        <field key="search" value={query} live={true} call={call("notes.search")} arg="q" hint="search" />
        <field key="draft" value="" live={false} call={call("notes.add")} arg="text" hint="write a note" />
      </card>
      {noteCards}
      <card key="footer">
        <button key="export" call={call("notes.export")}>export</button>
        <button key="clear" call={call("notes.clear")}>clear all</button>
      </card>
    </stack>
  )
}
