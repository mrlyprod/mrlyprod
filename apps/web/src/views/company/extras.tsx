import { call } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type Link = { name: string; url: string }

type State = {
  socials: Link[]
  actions: Link[]
  pages: Link[]
  cycle: { lines: string[]; index: number }
  copyright: string
}

export function extras(state: unknown, _send: Send): Node {
  const s = state as State
  return (
    <stack key="extras">
      <card key="socials">
        {s.socials.map(link => (
          <label key={link.name} mode="text" text={link.name} href={link.url} />
        ))}
      </card>
      <card key="actions">
        {s.actions.map(link => (
          <label key={link.name} mode="text" text={link.name} href={link.url} />
        ))}
      </card>
      <card key="pages">
        {s.pages.map(link => (
          <label key={link.name} mode="text" text={link.name} href={link.url} />
        ))}
      </card>
      <card key="cycle">
        <label key="line" mode="text" text={s.cycle.lines[s.cycle.index] ?? ""} call={call("extras.cycle")} fx="scramble" />
        <text key="copyright" role="note">{s.copyright}</text>
      </card>
    </stack>
  )
}
