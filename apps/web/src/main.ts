import * as gpu from "./gpu.ts"
import { act, boot, describe, frame, geometry, load, palette, peek, shaders } from "./kernel.ts"
import type { Journal } from "./journal.ts"
import { Store } from "./journal.ts"
import { install } from "./palette.ts"
import { theme } from "./render/theme.ts"
import { router } from "./router.ts"
import { mount } from "./shell/mount.ts"
import type { Call, Observation } from "./types.ts"

await load(fetch("/mrlyjs_bg.wasm"))
const handle = boot()
install(palette())
const registry = describe()
gpu.init(shaders(), route => geometry(handle, route))

const params = new URLSearchParams(location.search)
if (params.has("fresh")) {
  await Store.wipe("guest")
  params.delete("fresh")
  const query = params.toString()
  history.replaceState(null, "", location.pathname + (query === "" ? "" : `?${query}`))
}

const store = await Store.open("guest", registry.version)

if (store.journal.snapshot !== undefined) {
  act(handle, { verb: "sys.thaw", args: { state: store.journal.snapshot } })
}
for (const call of store.journal.calls) act(handle, call)
const looks = peek(handle, "settings")?.state as Record<string, unknown> | undefined
for (const [key, value] of Object.entries(looks ?? {})) theme(key, value)

const path = (obs: Observation) => {
  const app = obs.route?.app
  if (app === undefined) return "/"
  const state = obs.view?.state as { slug?: unknown } | undefined
  const slug = typeof state?.slug === "string" && state.slug !== "" ? `/${state.slug}` : ""
  return `/${app}${slug}`
}
const known = (app: string) => registry.apps.some(m => m.route === app)
const opens = (app: string) => registry.verbs.some(v => v.app === app && v.verbs.some(x => x.verb === `${app}.open`))

let atPath = path(frame(handle))
let quiet = false

const routes = router()

routes.after("settings.set", (_call, obs) => {
  if (obs.last?.ok) {
    const data = obs.last.data as { key: string; value: unknown }
    theme(data.key, data.value)
  }
})

const send = (call: Call, beat = false) => {
  if (routes.handle(call)) return
  const stamped = { ...call, now: Date.now() }
  store.record(stamped, beat)
  const obs = act(handle, stamped)
  routes.observe(call, obs)
  ui.render(obs)
  const next = path(obs)
  if (next !== atPath) {
    atPath = next
    if (!quiet) history.pushState(null, "", next)
  }
  quiet = false
  if (store.full()) {
    const frozen = act(handle, { verb: "sys.freeze", args: {}, now: Date.now() })
    if (frozen.last?.ok) store.compact(frozen.last.data)
  }
}

const ui = mount(document.getElementById("mrly") as HTMLElement, send, routes, registry.apps, app => peek(handle, app))

if (looks?.render === "gpu" && navigator.gpu === undefined) {
  send({ verb: "settings.set", args: { key: "render", value: "cpu" } })
}

routes.on("journal.reset", async () => {
  if (!(await ui.ask("wipe the session and reboot?"))) return
  await Store.wipe("guest")
  location.reload()
})
routes.on("journal.export", () => {
  const blob = new Blob([JSON.stringify(store.journal)], { type: "application/json" })
  const url = URL.createObjectURL(blob)
  const link = document.createElement("a")
  link.href = url
  link.download = "mrly-session.json"
  link.click()
  URL.revokeObjectURL(url)
})
routes.on("journal.import", () => {
  const input = document.createElement("input")
  input.type = "file"
  input.accept = "application/json,.json"
  input.onchange = () => {
    const file = input.files?.[0]
    if (file === undefined) return
    void file.text().then(async text => {
      let parsed: unknown
      try {
        parsed = JSON.parse(text)
      } catch {
        ui.pop("not a session file")
        return
      }
      const found = parsed as Journal
      if (typeof found.version !== "string" || !Array.isArray(found.calls)) {
        ui.pop("not a session file")
        return
      }
      if (found.version !== registry.version) {
        ui.pop(`session is from build ${found.version}, this is ${registry.version}`)
        return
      }
      if (!(await ui.ask("overwrite the current session?"))) return
      await Store.put({ ...found, iden: "guest" })
      history.replaceState(null, "", "/")
      location.reload()
    })
  }
  input.click()
})

if (store.discarded) ui.pop("stored session was from another build and was discarded")

const follow = (pathname: string) => {
  const landed = pathname.split("/").filter(part => part !== "")
  const target = landed[0]
  if (target === undefined) return
  const rest = landed.slice(1)
  if (!known(target)) return
  const now = frame(handle)
  if (target !== now.route?.app) {
    quiet = true
    send({ verb: "nav.open", args: { app: target } })
  }
  if (rest.length > 0 && opens(target)) {
    quiet = true
    send({ verb: `${target}.open`, args: { slug: rest.join("/") } })
  }
}

follow(location.pathname)
history.replaceState(null, "", atPath + location.search)

window.addEventListener("popstate", () => {
  if (location.pathname === atPath) return
  follow(location.pathname)
})

ui.render(frame(handle))
