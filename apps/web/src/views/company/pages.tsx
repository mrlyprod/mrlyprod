import { call } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Node, Send } from "../../types.ts"

type State = {
  slug: string
  md: string
  mode: string
  status: string
  source: string
}

export function pages(state: unknown, _send: Send): Node {
  const s = state as State
  const note =
    s.status === "loading"
      ? `landing ${s.slug}…`
      : s.status === "error"
        ? `${s.slug} would not land`
        : s.status === "empty"
          ? "no page open"
          : null
  return (
    <stack key="pages">
      <card key="toolbar">
        <grid key="tools" cols={s.source === "" ? 2 : 3}>
          <button key="flip" call={call("pages.flip")}>{s.mode === "preview" ? "code" : "preview"}</button>
          <button key="full" call={call("face.full", { handle: "pages" })}>fullscreen</button>
          {s.source !== "" && <label key="source" mode="text" text="source" href={s.source} />}
        </grid>
      </card>
      {note !== null && (
        <card key="status">
          <text key="note" role="note">{note}</text>
        </card>
      )}
      {s.status === "empty" && (
        <card key="hint">
          <button key="dummy" call={call("pages.open", { slug: "dummy" })}>open the dummy page</button>
        </card>
      )}
      {s.status === "ready" && (
        <doc key="doc" handle="pages" md={s.md} code={s.mode === "code" ? s.md : undefined} open={call("pages.open")} />
      )}
    </stack>
  )
}
