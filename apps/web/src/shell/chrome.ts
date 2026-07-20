import { spell } from "../glyphs.ts"
import { make } from "../render/paint.ts"
import type { Manifest, Observation, Send } from "../types.ts"

export function chrome(emit: Send, current: () => Observation | null, apps: Manifest[]) {
  const glyph = (name: string, text: string) => {
    const el = make("button", `mrly-box link-box ${name}`)
    spell(el, text)
    return el
  }
  const bar = make("header", "mrly-box border-box bar")
  const menu = glyph("menu", "+")
  menu.addEventListener("click", () => emit({ verb: "nav.open", args: { app: "menu" } }))
  const mrly = glyph("mrly", "X")
  mrly.addEventListener("click", () => emit({ verb: "splash.on", args: {} }))
  const iden = glyph("iden", "O")
  iden.addEventListener("click", () => emit({ verb: "nav.open", args: { app: "iden" } }))
  bar.append(menu, mrly, iden)
  const title = make("div", "mrly-box border-box title")
  const label = make("button", "mrly-box link-box info-box")
  label.addEventListener("click", () => {
    const view = current()?.view ?? null
    if (view === null) return
    const reset = view.actions.some(v => v.verb === `${view.app}.reset`)
    emit(reset ? { verb: `${view.app}.reset`, args: {} } : { verb: "nav.open", args: { app: view.app } })
  })
  title.append(label)
  const drawChrome = () => {
    const app = current()?.view?.app
    spell(label, (apps.find(a => a.route === app)?.title ?? app ?? "").toUpperCase())
  }
  return { bar, title, drawChrome }
}
