import type { Call, Manifest, Node, Send, View } from "../types.ts"
import { fallback, views } from "../views/index.ts"

export type Peek = (app: string) => View | null

export function picker(send: Send, peek: Peek | undefined, apps: Manifest[], repaint: () => void) {
  let picking: { app: string; host: string; key: string } | null = null
  const active = () => picking !== null
  const pick = (call: Call) => {
    if (call.verb === "pick.open") {
      const args = call.args as { app?: string; host?: string; key?: string; value?: unknown }
      if (typeof args.app !== "string" || typeof args.host !== "string" || typeof args.key !== "string") return
      picking = { app: args.app, host: args.host, key: args.key }
      if (args.value !== undefined && args.value !== null) {
        send({ verb: `${args.app}.set`, args: { key: "work", value: args.value } })
      } else {
        repaint()
      }
    }
    if (call.verb === "pick.close") {
      picking = null
      repaint()
    }
    if (call.verb === "pick.apply") {
      if (picking === null || peek === undefined) return
      const staged = (peek(picking.app)?.state as { work?: unknown } | undefined)?.work
      const target = picking
      picking = null
      if (staged !== undefined) {
        send({ verb: `${target.host}.set`, args: { key: target.key, value: staged } })
      } else {
        repaint()
      }
    }
  }
  const build = (emit: Send): Node[] => {
    if (picking === null || peek === undefined) return []
    const hosted = peek(picking.app)
    if (hosted === null) {
      picking = null
      return []
    }
    const draw = views[hosted.app]
    const body = draw !== undefined ? draw(hosted.state, emit) : fallback(hosted.app, hosted.state, apps.find(a => a.route === hosted.app))
    return [
      {
        kind: "Overlay",
        key: "picker",
        close: { verb: "pick.close", args: {} },
        child: {
          kind: "Stack",
          key: "picker-stack",
          children: [
            body,
            {
              kind: "Card",
              key: "picker-actions",
              children: [
                { kind: "Button", key: "apply", label: "apply", call: { verb: "pick.apply", args: {} } },
                { kind: "Button", key: "cancel", label: "cancel", call: { verb: "pick.close", args: {} } },
              ],
            },
          ],
        },
      },
    ]
  }
  return { active, pick, build }
}
