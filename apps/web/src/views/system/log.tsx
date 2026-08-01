import { call } from "../../builders.ts"
import { Section } from "../../components/Section.tsx"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type Entry = { verb: string; args: unknown; now: number; tick: number }

const show = (args: unknown): string => {
  const body = JSON.stringify(args)
  if (body === "{}") return ""
  return body.length > 80 ? `${body.slice(0, 77)}...` : body
}

export function log(state: unknown, _send: Send): Node {
  const s = state as { entries: Entry[] }
  return (
    <stack key="log">
      <Section keyName="calls" label="calls">
        {s.entries.length === 0 && <text key="empty" role="note">no calls yet</text>}
        {s.entries.map(e => (
          <text key={`entry-${e.tick}`}>{`${e.tick}  ${e.verb}  ${show(e.args)}`.trimEnd()}</text>
        ))}
      </Section>
      <card key="footer">
        <button key="export" call={call("log.export")}>export</button>
      </card>
    </stack>
  )
}
