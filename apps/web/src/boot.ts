import * as gpu from "mrlygpu"
import { toast } from "mrlyui"
import { ask } from "./ask"
import { aim } from "./eyes/orbit"
import { look } from "./eyes/theme"
import paletteData from "./gen/palette.json"
import shadersData from "./gen/shaders.json"
import { boot as wake, buffer, call, list, load, observe, read, view } from "./kernel"
import type { Journal } from "./journal"
import { Store } from "./journal"
import { board, install } from "./palette"
import { install as installReads } from "./reads"
import * as pwa from "./pwa"
import { improvise } from "./roller"
import { router } from "./router"
import { shoot } from "./components/Shot"
import { setSplash } from "./splash"
import type { Observation, Palette, Registry, Send, Shaders } from "./types"

export type Face = {
  registry: Registry
  send: Send
  stray: () => void
  full: (handle: string) => void
  subscribe: (fn: () => void) => () => void
  snapshot: () => Observation
}

const KERNEL = new URL("../../../pkgs/mrlyjs/web/pkg/mrlyjs_bg.wasm", import.meta.url)

export async function boot(): Promise<Face> {
  await load(fetch(KERNEL))
  const handle = wake("arcade")
  install(paletteData as Palette)
  installReads(path => read(handle, path))
  const registry = list(handle)
  gpu.init(shadersData as Shaders, {
    geometry: route => buffer(handle, `${route}/geometry`),
    uniforms: route => aim(route, buffer(handle, `${route}/uniforms`)),
    board,
  })

  const params = new URLSearchParams(location.search)
  if (params.has("fresh")) {
    await Store.wipe("guest")
    params.delete("fresh")
    const query = params.toString()
    history.replaceState(null, "", location.pathname + (query === "" ? "" : `?${query}`))
  }

  const store = await Store.open("guest", registry.version)

  if (store.journal.snapshot !== undefined) {
    call(handle, { verb: "sys.thaw", args: { state: store.journal.snapshot } })
  }
  for (const kept of store.journal.calls) call(handle, kept)
  const looks = view(handle, "settings")?.state as Record<string, unknown> | undefined
  for (const [key, value] of Object.entries(looks ?? {})) look(key, value)

  const path = (obs: Observation) => {
    const app = obs.route?.app
    if (app === undefined) return "/"
    const state = obs.view?.state as { slug?: unknown } | undefined
    const slug = typeof state?.slug === "string" && state.slug !== "" ? `/${state.slug}` : ""
    return `/${app}${slug}`
  }
  const known = (app: string) => registry.apps.some(m => m.route === app)
  const opens = (app: string) => registry.verbs.some(v => v.app === app && v.verbs.some(x => x.verb === `${app}.open`))

  let atPath = path(observe(handle))
  let quiet = false

  const routes = router()

  routes.after("settings.set", (_call, obs) => {
    if (obs.last?.ok) {
      const data = obs.last.data as { key: string; value: unknown }
      look(data.key, data.value)
    }
  })

  let held: Observation = observe(handle)
  const subs = new Set<() => void>()
  const publish = (obs: Observation) => {
    held = obs
    for (const sub of subs) sub()
  }

  const send: Send = (made, beat = false) => {
    if (routes.handle(made)) return
    const shot = made.verb === "sys.shot" ? shoot(made) : made
    const stamped = { ...shot, now: Date.now() }
    store.record(stamped, beat)
    const obs = call(handle, stamped)
    routes.observe(shot, obs)
    publish(obs)
    const next = path(obs)
    if (next !== atPath) {
      atPath = next
      if (!quiet) history.pushState(null, "", next)
    }
    quiet = false
    if (store.full()) {
      const frozen = call(handle, { verb: "sys.freeze", args: {}, now: Date.now() })
      if (frozen.last?.ok) store.compact(frozen.last.data)
    }
  }

  const stray = () => {
    const made = improvise(held.view)
    if (made !== null) publish(call(handle, made))
  }

  const full = (target: string) => {
    const el = document.querySelector<HTMLElement>(`[data-handle="${target}"]`)
    if (el !== null) void el.requestFullscreen()
  }

  routes.on("face.full", made => {
    const target = (made.args as { handle?: unknown }).handle
    if (typeof target === "string") full(target)
  })
  routes.on("splash.", made => setSplash(made.verb === "splash.on"))
  pwa.watch()
  routes.on("device.install", () => {
    void pwa.install().then(note => toast(note))
  })
  routes.on("journal.reset", async () => {
    if (!(await ask("wipe the session and reboot?"))) return
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
          toast("not a session file", "danger")
          return
        }
        const found = parsed as Journal
        if (typeof found.version !== "string" || !Array.isArray(found.calls)) {
          toast("not a session file", "danger")
          return
        }
        if (found.version !== registry.version) {
          toast(`session is from build ${found.version}, this is ${registry.version}`, "warn")
          return
        }
        if (!(await ask("overwrite the current session?"))) return
        await Store.put({ ...found, iden: "guest" })
        history.replaceState(null, "", "/")
        location.reload()
      })
    }
    input.click()
  })

  if (store.discarded) toast("stored session was from another build and was discarded", "warn")

  const follow = (pathname: string) => {
    const landed = pathname.split("/").filter(part => part !== "")
    const target = landed[0]
    if (target === undefined) return
    const rest = landed.slice(1)
    if (!known(target)) return
    if (target !== held.route?.app) {
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

  publish(observe(handle))

  return {
    registry,
    send,
    stray,
    full,
    subscribe: fn => {
      subs.add(fn)
      return () => subs.delete(fn)
    },
    snapshot: () => held,
  }
}
