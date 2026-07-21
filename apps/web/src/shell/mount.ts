import { ColorPicker } from "../components/ColorPicker.tsx"
import { GlyphPicker } from "../components/GlyphPicker.tsx"
import { TilePicker } from "../components/TilePicker.tsx"
import { TimePicker } from "../components/TimePicker.tsx"
import { designs, glyphs } from "../kernel.ts"
import { prune } from "../render/boards.ts"
import { markCanvas } from "../render/mark.ts"
import { make, paint } from "../render/paint.ts"
import { reconcile } from "../render/reconcile.ts"
import type { Router } from "../router.ts"
import { buzz, tick, unlock } from "../sound.ts"
import type { Call, Designs, GlyphSet, Manifest, Node, Observation, Send } from "../types.ts"
import { fallback, keys, views } from "../views/index.ts"
import { chrome } from "./chrome.ts"
import { perform } from "./effects.ts"

type Picking =
  | { kind: "color"; host: string; key: string; value?: string }
  | { kind: "glyph"; host: string; key: string; sets: GlyphSet[]; cat: number; value?: string }
  | { kind: "time"; host: string; key: string; h: number; m: number }
  | { kind: "tile"; host: string; key: string; data: Designs }

export type Ui = {
  render: (obs: Observation) => void
  pop: (text: string) => void
  ask: (text: string) => Promise<boolean>
}

export function mount(root: HTMLElement, send: Send, routes: Router, apps: Manifest[]): Ui {
  root.className = "mrly splashed"
  let pending: Call | null = null
  let picking: Picking | null = null
  routes.on("face.full", call => {
    const handle = (call.args as { handle?: unknown }).handle
    if (typeof handle === "string") {
      const el = root.querySelector<HTMLElement>(`[data-handle="${handle}"]`)
      if (el !== null) void el.requestFullscreen()
    }
  })
  const emit: Send = (call, beat) => {
    if (routes.handle(call)) return
    pending = call
    send(call, beat)
  }

  const splash = make("div", "splash")
  splash.addEventListener("click", () => emit({ verb: "splash.off", args: {} }))
  routes.on("splash.", call => {
    root.classList.toggle("splashed", call.verb === "splash.on")
  })

  const ARROWS: Record<string, "up" | "down" | "left" | "right"> = {
    ArrowUp: "up",
    ArrowDown: "down",
    ArrowLeft: "left",
    ArrowRight: "right",
    w: "up",
    a: "left",
    s: "down",
    d: "right",
    W: "up",
    A: "left",
    S: "down",
    D: "right",
  }
  window.addEventListener("keydown", event => {
    if (event.ctrlKey || event.metaKey || event.altKey) return
    const tag = document.activeElement?.tagName ?? ""
    if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return
    if (picking !== null || current === null || root.classList.contains("splashed")) return
    const dir = ARROWS[event.key]
    if (dir === undefined) return
    const app = current.view?.app
    if (app === undefined) return
    const hit = keys[app]?.[dir]
    if (hit === undefined) return
    event.preventDefault()
    emit(hit)
  })

  const { bar, title, drawChrome } = chrome(emit, () => current, apps)

  const column = make("div", "column")
  const view = make("div", "view list-box")
  column.append(view)
  const foot = make("footer", "mrly-box border-box")
  foot.append(markCanvas())

  const sheet = make("div", "sheet")

  const toast = make("div", "mrly-box border-box toast")
  const notice = make("p", "mrly-box info-box")
  toast.append(notice)
  toast.hidden = true
  let timer: ReturnType<typeof setTimeout> | undefined
  const pop = (text: string) => {
    notice.textContent = text
    toast.hidden = false
    clearTimeout(timer)
    timer = setTimeout(() => {
      toast.hidden = true
    }, 4000)
  }

  let asking: { text: string; resolve: (ok: boolean) => void } | null = null
  const asked = (): Node[] => {
    if (asking === null) return []
    return [
      {
        kind: "Overlay",
        key: "ask",
        close: { verb: "ask.no", args: {} },
        child: {
          kind: "Card",
          key: "ask-card",
          children: [
            { kind: "Text", key: "ask-text", text: asking.text },
            { kind: "Button", key: "ask-yes", label: "yes", call: { verb: "ask.yes", args: {} } },
            { kind: "Button", key: "ask-no", label: "no", call: { verb: "ask.no", args: {} } },
          ],
        },
      },
    ]
  }
  const pickerNodes = (): Node[] => {
    const p = picking
    if (p === null) return []
    const close: Call = { verb: "sheet.close", args: {} }
    const onpick = (value: unknown): Call => ({ verb: `${p.host}.set`, args: { key: p.key, value } })
    const bend = (key: string, value: unknown): Call => ({ verb: "sheet.turn", args: { key, value } })
    if (p.kind === "color") return [ColorPicker({ value: p.value, onpick, close })]
    if (p.kind === "glyph") return [GlyphPicker({ sets: p.sets, cat: p.cat, value: p.value, onpick, turn: cat => bend("cat", cat), close })]
    if (p.kind === "time") return [TimePicker({ h: p.h, m: p.m, turn: (key, value) => bend(key, value), onpick, close })]
    return [TilePicker({ data: p.data, turn: bend, onpick, close })]
  }
  const answer: Send = call => {
    if (call.verb.startsWith("ask.")) {
      const done = asking
      asking = null
      drawSheet()
      done?.resolve(call.verb === "ask.yes")
      return
    }
    emit(call)
    if (!call.verb.startsWith("sheet.") && picking !== null) {
      picking = null
      drawSheet()
    }
  }
  routes.on("sheet.", call => {
    const a = call.args as Record<string, unknown>
    if (call.verb === "sheet.open") {
      const host = a.host as string
      const key = a.key as string
      if (a.picker === "color") picking = { kind: "color", host, key, value: a.value as string | undefined }
      else if (a.picker === "glyph") picking = { kind: "glyph", host, key, sets: glyphs(a.set as string), cat: 0, value: a.value as string | undefined }
      else if (a.picker === "time") {
        const v = (a.value ?? { h: 0, m: 0 }) as { h: number; m: number }
        picking = { kind: "time", host, key, h: v.h, m: v.m }
      } else if (a.picker === "tile") picking = { kind: "tile", host, key, data: designs({}) }
      drawSheet()
      return
    }
    if (call.verb === "sheet.turn" && picking !== null) {
      const key = a.key as string
      if (picking.kind === "glyph" && key === "cat") picking = { ...picking, cat: a.value as number }
      else if (picking.kind === "time" && (key === "h" || key === "m")) picking = { ...picking, [key]: a.value as number }
      else if (picking.kind === "tile") {
        const cfg = picking.data.config
        const req = { group: cfg.group, catalog: cfg.catalog, page: cfg.page, dark: cfg.dark }
        if (key === "group") picking = { ...picking, data: designs({ ...req, group: a.value as string, page: 0 }) }
        else if (key === "catalog") picking = { ...picking, data: designs({ ...req, catalog: a.value as string, page: 0 }) }
        else if (key === "page") picking = { ...picking, data: designs({ ...req, page: a.value as number }) }
      }
      drawSheet()
      return
    }
    if (call.verb === "sheet.close") {
      picking = null
      drawSheet()
    }
  })
  const drawSheet = () => reconcile(sheet, [...pickerNodes(), ...asked()], answer)
  const ask = (text: string): Promise<boolean> =>
    new Promise(resolve => {
      asking?.resolve(false)
      asking = { text, resolve }
      drawSheet()
    })

  root.replaceChildren(bar, title, column, foot, sheet, toast, splash)

  root.addEventListener("pointerdown", () => unlock())
  root.addEventListener("click", event => {
    const hit = (event.target as HTMLElement).closest(".link-box") as HTMLElement | null
    if (hit !== null && hit.dataset.press === undefined) tick()
  })

  let current: Observation | null = null

  const beat = () => {
    if (root.classList.contains("splashed")) return
    prune()
    const pulse = current?.view?.beat
    if (pulse !== undefined) emit(pulse, true)
  }
  setInterval(beat, 125)

  let seen = -1
  let topo = ""

  const repaint = (obs: Observation) => {
    for (const effect of obs.effects ?? []) perform(effect, emit)

    bar.style.setProperty("--tint", paint(bar))
    title.style.setProperty("--tint", paint(title))
    drawChrome()

    const slot = obs.view
    if (slot === null) reconcile(view, [], emit)
    else {
      const draw = views[slot.app]
      reconcile(view, [draw !== undefined ? draw(slot.state, emit) : fallback(slot.app, slot.state, apps.find(a => a.route === slot.app))], emit)
    }
    foot.style.setProperty("--tint", paint(foot))

    const notices = obs.notices ?? []
    if (seen >= 0 && notices.length > seen) {
      const notice = notices[notices.length - 1] as { title: string; body: string }
      pop(notice.body === "" ? notice.title : `${notice.title} · ${notice.body}`)
    }
    seen = notices.length

    if (pending !== null) {
      pending = null
      const failed = obs.last !== null && !obs.last.ok
      if (failed) buzz(30)
    }

    drawSheet()
  }

  const render = (obs: Observation) => {
    current = obs
    const next = obs.view?.app ?? ""
    if (topo !== "" && next !== topo) window.scrollTo(0, 0)
    topo = next
    repaint(obs)
  }

  return { render, pop, ask }
}
