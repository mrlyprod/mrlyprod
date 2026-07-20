import { call } from "../../builders.ts"
import { h } from "../../jsx.ts"
import type { Manifest, Node, Send } from "../../types.ts"

type State = { apps: Manifest[]; query: string; mode: "grid" | "list" | "carousel" }

const GROUPS = ["system", "tools", "creativity", "design", "math", "physics", "puzzles", "games", "toys", "company"]

export function menu(state: unknown, _send: Send): Node {
  const { apps, query, mode } = state as State
  const layout: "row" | "stack" = mode === "list" ? "row" : "stack"
  const cols = mode === "list" ? 1 : 3
  const top = apps[0]

  const cards = apps.map(app => (
    <card key={app.route}>
      <label
        key={app.route}
        mode={layout}
        symbol={{ as: "emoji", value: app.emoji }}
        text={app.title}
        call={call("nav.open", { app: app.route })}
      />
    </card>
  ))

  return (
    <stack key="menu">
      <card key="search">
        <field
          key="search"
          value={query}
          live={true}
          call={call("menu.search")}
          arg="q"
          hint="search"
          icon="search"
          clear={true}
          enter={top !== undefined ? call("nav.open", { app: top.route }) : undefined}
        />
      </card>
      {query === "" && (
        <card key="chips">
          <pills key="groups">
            {GROUPS.map(group => (
              <label key={group} mode="text" text={group} call={call("menu.search", { q: group })} />
            ))}
          </pills>
        </card>
      )}
      <grid key="apps" cols={cols} mode={mode === "carousel" ? "snap" : undefined}>
        {cards}
      </grid>
      <card key="footer">
        <label key="privacy" mode="text" text="privacy" href="/privacy" />
        <label key="terms" mode="text" text="terms" href="/terms" />
      </card>
    </stack>
  )
}
