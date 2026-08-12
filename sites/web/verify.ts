import { readFileSync, readdirSync } from "node:fs"
import { createElement } from "react"
import { renderToStaticMarkup } from "react-dom/server"
import markData from "../../pkgs/js/mrlyui/src/gen/mark.json"
import shadersData from "./src/gen/shaders.json"
import skinsData from "./src/gen/skins.json"
import type { Journal } from "./src/journal"
import { RUNS, stops, upto } from "./src/journal"
import type { Handle } from "./src/kernel"
import { boot, buffer, call, list, load, observe, read } from "./src/kernel"
import { install as installReads } from "./src/reads"
import { plays } from "./src/roller"
import { Kernel } from "./src/send"
import type { Call, Mark, Observation, Shade, Shaders, Skins } from "./src/types"
import { views } from "./src/views/index"

const wasm = readFileSync(new URL("../../pkgs/js/web/pkg/mrlyweb_bg.wasm", import.meta.url))
await load(wasm)

const handle = boot("full")
installReads(path => read(handle, path))
const registry = list(handle)
let now = 1783600496000
let failures = 0

function check(name: string, ok: boolean, detail = "") {
  if (!ok) failures += 1
  console.log(`${ok ? "ok  " : "FAIL"} ${name}${detail === "" ? "" : ` (${detail})`}`)
}

function send(verb: string, args: Call["args"] = {}): Observation {
  now += 1000
  return call(handle, { verb, args, now })
}

function look(): Observation {
  return observe(handle)
}

function focused(obs: Observation): Record<string, unknown> {
  return obs.view?.state as Record<string, unknown>
}

function state(): Record<string, unknown> {
  return focused(look())
}

function visit(app: string): Observation {
  return send("nav.open", { app })
}

// THE ALTITUDE LAW

const VOID = new Set(["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track", "wbr"])

const CONTROLS = new Set(["button", "input", "select", "textarea", "canvas"])

const DEEP = 3

function classes(tag: string): string {
  const found = /class="([^"]*)"/.exec(tag)
  return found?.[1] ?? ""
}

function surface(tag: string): boolean {
  const worn = classes(tag).split(" ")
  return worn.includes("box") || worn.includes("card") || worn.includes("section")
}

function altitude(tag: string): boolean {
  return classes(tag).split(" ").includes("box")
}

function law(markup: string): string[] {
  const faults: string[] = []
  const stack: { name: string; box: boolean; roof: boolean }[] = []
  for (const found of markup.matchAll(/<\/?([a-zA-Z][a-zA-Z0-9-]*)([^>]*)>/g)) {
    const whole = found[0]
    const name = (found[1] ?? "").toLowerCase()
    if (whole.startsWith("</")) {
      while (stack.length > 0 && stack.pop()?.name !== name) continue
      continue
    }
    const box = altitude(whole)
    const roof = surface(whole)
    const boxes = stack.filter(one => one.box).length
    if (box && boxes >= DEEP) faults.push(`deep ${name}.${classes(whole)}`)
    if (CONTROLS.has(name) && !roof && !stack.some(one => one.roof)) faults.push(`naked ${name}`)
    if (VOID.has(name) || whole.endsWith("/>")) continue
    stack.push({ name, box, roof })
  }
  return faults
}

function paint(app: string, worn: unknown): string {
  const draw = views[app]
  if (draw === undefined) return ""
  return renderToStaticMarkup(
    createElement(Kernel.Provider, { value: registry }, createElement(draw, { state: worn })),
  )
}

const boxless: string[] = []

function checkBox(route: string, markup: string) {
  const faults = law(markup)
  if (faults.length > 0) boxless.push(`${route}: ${faults.join(", ")}`)
}

function tally(markup: string, tag: string): number {
  return markup.split(`<${tag}`).length - 1
}

const booted = observe(handle)
check("boots at menu, tick 0", booted.route?.app === "menu" && booted.tick === 0)
check("registry names a version", typeof registry.version === "string" && registry.version !== "")

const anim = markData as Mark
check(
  "the mark animation is baked",
  anim.rows === 7 && anim.cols === 49 && anim.fps === 25 && anim.frames.length > 100,
  `${anim.frames.length} frames`,
)

const programs = shadersData as Shaders
check(
  "the shader programs are baked",
  Object.keys(programs).length > 0 && Object.values(programs).every(source => source.includes("fn vs_main") && source.includes("fn fs_main")),
  Object.keys(programs).join(" "),
)

const skins = skinsData as Skins
check(
  "the app skins are baked",
  Object.keys(skins).length >= 12 && (skins["ttt"]?.["tiles"]?.length ?? 0) > 0 && (skins["twenty48"]?.["digits"]?.length ?? 0) > 0,
  Object.keys(skins).join(" "),
)

visit("calculator")
for (const d of [6]) send("calculator.digit", { d })
send("calculator.op", { op: "mul" })
send("calculator.digit", { d: 7 })
const equals = send("calculator.equals")
check("calculator reads 42 after 6 * 7 =", state()["display"] === "42", String(state()["display"]))
check("tick advances with every call", equals.tick > 4, String(equals.tick))

visit("notes")
send("notes.add", { text: "buy oat milk" })
send("notes.add", { text: "book the ferry" })
const found = () => (state()["found"] as unknown[]).length
check("notes holds two cards", found() === 2, String(found()))
send("notes.search", { q: "ferry" })
check("search narrows to one card", found() === 1, String(found()))
send("notes.search", { q: "" })
send("notes.remove", { id: (state()["found"] as { id: unknown }[])[0]?.id })
check("remove leaves one card", found() === 1, String(found()))

visit("settings")
send("settings.set", { key: "color", value: "mint" })
check("settings.set lands in state", state()["color"] === "mint")
const bad = send("settings.set", { key: "color", value: "beige" })
check("settings.set rejects garbage honestly", bad.last?.ok === false)
send("settings.set", { key: "wave", value: "square" })
check("settings.set lands a wave", state()["wave"] === "square")
const noise = send("settings.set", { key: "wave", value: "noise" })
check("settings.set rejects a foreign wave", noise.last?.ok === false)
const slow = send("settings.set", { key: "duration", value: 5000 })
check("settings.set holds the duration bounds", slow.last?.ok === false)
send("settings.set", { key: "launchpad", value: "list" })
check("settings.set lands the launchpad", state()["launchpad"] === "list")
const pad = send("settings.set", { key: "launchpad", value: "carousel" })
check("settings.set holds the launchpad bounds", pad.last?.ok === false)
send("settings.set", { key: "launchpad", value: "grid" })
send("settings.set", { key: "material", value: "glass" })
check("settings.set lands the material", state()["material"] === "glass")
const frosted = send("settings.set", { key: "material", value: "frosted" })
check("settings.set holds the material options", frosted.last?.ok === false)
send("settings.set", { key: "material", value: "solid" })
send("settings.set", { key: "wallpaper", value: "pattern" })
check("settings.set lands the wallpaper", state()["wallpaper"] === "pattern")
const mural = send("settings.set", { key: "wallpaper", value: "mural" })
check("settings.set holds the wallpaper options", mural.last?.ok === false)
send("settings.set", { key: "wallpaper", value: "color" })
send("settings.set", { key: "seed", value: 42 })
check("settings.set lands the seed", state()["seed"] === 42)
const wild = send("settings.set", { key: "seed", value: 5000 })
check("settings.set holds the seed bounds", wild.last?.ok === false)
send("settings.set", { key: "seed", value: 0 })

const advertised = Object.keys(
  (registry.verbs.find(v => v.app === "settings")?.verbs.find(v => v.verb === "settings.set")?.args["key"] ?? {}) as Record<string, unknown>,
)
check("settings.set advertises a menu of keys", advertised.length >= 10, String(advertised.length))
const deaf: string[] = []
for (const key of advertised) {
  if (send("settings.set", { key, value: state()[key] }).last?.ok !== true) deaf.push(key)
}
check("every advertised settings key answers to settings.set", deaf.length === 0, deaf.join(", "))
const phantom = send("settings.set", { key: "phantom", value: 1 })
check("an unadvertised settings key fails", phantom.last?.ok === false)

visit("studio")
const pressed = send("studio.press", { midi: 43 })
const tone = pressed.effects?.[0] as { kind: string; data: { op?: string; id?: string; wave?: string } } | undefined
check(
  "studio.press holds the key and starts a sound",
  (state()["held"] as number[]).includes(43) && tone?.kind === "sound" && tone.data.op === "start" && tone.data.id === "studio:43",
)
check("the pressed key rings the worn wave", tone?.data.wave === "square", String(tone?.data.wave))
const lifted = send("studio.lift", { midi: 43 })
const hush = lifted.effects?.[0] as { kind: string; data: { op?: string; id?: string } } | undefined
check(
  "studio.lift releases the key and stops the sound",
  (state()["held"] as number[]).length === 0 && hush?.kind === "sound" && hush.data.op === "stop" && hush.data.id === "studio:43",
)
const unheld = send("studio.lift", { midi: 43 })
check("an orphan lift fails honestly", unheld.last?.ok === false)
const keyboard = paint("studio", focused(look()))
check("the studio hangs its keys", tally(keyboard, "button") >= 21, String(tally(keyboard, "button")))
visit("settings")
send("settings.set", { key: "wave", value: "sine" })

send("settings.set", { key: "font", value: "mrly" })
const worn = visit("calculator")
check("the worn face keeps a plain state", focused(worn)["glyph"] === undefined && focused(worn)["display"] === "42")
const readout = paint("calculator", focused(worn))
check("the readout spells the display as glyphs", readout.includes('aria-label="42"'))
checkBox("calculator (worn)", readout)
visit("settings")
send("settings.set", { key: "font", value: "mono" })
visit("calculator")
check("flag down clears the raster", state()["glyph"] === undefined)

visit("fonts")
send("fonts.pick", { char: "a" })
check("fonts picks a glyph", state()["char"] === "a", String(state()["char"]))
const lit = () => ((state()["glyph"] as { rows: number[][] }).rows.flat().filter(v => v !== 0).length)
const full = lit()
send("fonts.scramble")
const dark = lit()
send("fonts.tick")
check("the scramble reveals pixel by pixel", full > 0 && dark === 0 && lit() === 1, `${full} ${dark} ${lit()}`)

const painting = visit("paint")
const paintMarkup = paint("paint", focused(painting))
check("paint hangs a canvas", tally(paintMarkup, "canvas") > 0)
checkBox("paint", paintMarkup)
send("paint.stroke", { points: [[0, 0], [1, 1]] })
check("a gesture lands as one stroke", state()["painted"] === 2 && state()["steps"] === 1)

visit("snake")
send("snake.reset", { seed: 7 })
send("snake.turn", { dir: "left" })
const stepped = send("snake.step", { n: 3 })
check("snake steps under its natural verbs", state()["steps"] === 3 && state()["over"] === false)
check("the beat is the step call", stepped.view?.beat?.verb === "snake.step")

const frozen = send("sys.freeze")
const snapshot = frozen.last?.data
send("snake.step", { n: 2 })
send("sys.thaw", { state: snapshot })
check("freeze and thaw restore the round", state()["steps"] === 3, String(state()["steps"]))

const living = visit("life")
const lifeMarkup = paint("life", focused(living))
const settings = () => state()["settings"] as Record<string, unknown>
check("life hangs a canvas", tally(lifeMarkup, "canvas") > 0)
checkBox("life", lifeMarkup)
const stepping = send("life.step", { n: 6 })
check("life steps its timeline forward", state()["generation"] === 6 && (state()["length"] as number) >= 7)
check("the life beat is the step call", stepping.view?.beat?.verb === "life.step")
send("life.run", { on: true })
check("run arms the beat", state()["running"] === true)
send("life.back")
check("back scrubs one frame and pauses", state()["cursor"] === 5 && state()["running"] === false)
send("life.start")
check("start rewinds to the oldest frame", state()["cursor"] === 0 && state()["generation"] === 0)
send("life.end")
check("end returns to the frontier", state()["cursor"] === (state()["length"] as number) - 1)
send("life.set", { key: "size", value: 24 })
check("resizing rebuilds the board", settings()["size"] === 24 && state()["generation"] === 0 && state()["length"] === 1)
const born = (settings()["birth"] as number[]).includes(2)
send("life.rule", { which: "birth", n: 2, on: !born })
check("a birth chip toggles membership", (settings()["birth"] as number[]).includes(2) === !born)
send("life.fill", { which: "survive", seq: "odds" })
check("a sequence fills the survive set", JSON.stringify(settings()["survive"]) === JSON.stringify([3, 5, 7]))
send("life.reset", { pattern: "soup" })
check("soup seeds a living board", (state()["population"] as number) > 0 && state()["generation"] === 0)
const tileLibrary = read(handle, "tile/library") as { value: unknown }[]
check("tile keeps a non-empty library", tileLibrary.length > 0, String(tileLibrary.length))
visit("life")
const picked = send("life.set", { key: "seed", value: tileLibrary[0]?.value })
check("a saved tile seeds the board", picked.last?.ok === true && state()["length"] === 1)
send("life.run", { on: false })
send("life.paint", { points: [[0, 0]] })
check("painting toggles a cell while paused", (state()["population"] as number) >= 1 && state()["fate"] === null)

visit("tile")
send("tile.set", { key: "group", value: "Magic" })
const shaped = () => state() as { tile: { group: string }; paint: unknown }
check("tile shapes a magic structure", shaped().tile.group === "Magic", shaped().tile.group)
send("tile.paint", { seed: 7 })
check("the paint dice lands a coat", shaped().paint !== null)
send("tile.reset")
check("tile.reset clears the studio", shaped().paint === null)
const tileMarkup = paint("tile", focused(look()))
check("the tile view hangs a preview canvas", tally(tileMarkup, "canvas") > 0)
checkBox("tile", tileMarkup)

visit("graph")
send("graph.set", { key: "kind", value: "tunnel" })
send("graph.set", { key: "fill", value: "mint" })
send("graph.set", { key: "ghost", value: true })
send("graph.page", { dir: "next" })
const recorded = (name: string): Record<string, unknown> => {
  const fixed = JSON.parse(readFileSync(new URL(`./fixtures/${name}.json`, import.meta.url), "utf8")) as Observation
  return fixed.view?.state as Record<string, unknown>
}
const census = JSON.stringify(state()["census"])
check("the graph census reads the same on wasm as it recorded on native", census === JSON.stringify(recorded("graph")["census"]), census)

const alive = boot("full")
let beat = now
const live = (verb: string, args: Call["args"] = {}): Observation => {
  beat += 1000
  return call(alive, { verb, args, now: beat })
}
live("nav.open", { app: "life" })
live("life.step")
const grown = focused(live("life.step"))["entropy"]
check("life's entropy reads the same on wasm as it recorded on native", grown === recorded("life")["entropy"], String(grown))

const kept = (app: string) => read(handle, `${app}/library`) as unknown[]
check("colors keeps the full name library", kept("colors").length === 15, String(kept("colors").length))
check("emojis keeps a non-empty library", kept("emojis").length > 0, String(kept("emojis").length))
check("fonts keeps a non-empty library", kept("fonts").length > 0, String(kept("fonts").length))
visit("tile")
send("tile.roll")
const saved = send("tile.save")
check("tile.save keeps the current tile", saved.last?.ok === true, String(saved.last?.note))

visit("photos")
const door = send("photos.load")
check("the picsum door is gone", door.last?.ok === false)
visit("life")
send("sys.shot", { image: { width: 2, height: 2, rows: [[0, 1], [1, 0]], palette: ["#000000", "#ffffff"] } })
visit("photos")
const wall = state()["photos"] as { rows: number[][]; palette: string[] }[]
check("a shot hangs a grid on the wall", wall.length === 1 && Array.isArray(wall[0]?.rows) && Array.isArray(wall[0]?.palette))
check("the wall needs no internet", registry.apps.find(a => a.route === "photos")?.internet !== true)

check("nav carries the open verb", registry.nav.length === 1 && registry.nav[0]?.verb === "nav.open", registry.nav.map(v => v.verb).join(" "))
const swapped = send("nav.open", { app: "menu" })
check("nav.open replaces the view", swapped.view?.app === "menu" && swapped.route?.app === "menu")
const ghost = send("nav.open", { app: "ghost" })
check("a missing app is refused", ghost.last?.ok === false && ghost.view?.app === "menu", String(ghost.last?.note))

const viewless: string[] = []
const broken: string[] = []
const misshaded: string[] = []
const unforwarded: string[] = []
for (const app of registry.apps) {
  const obs = visit(app.route)
  if (views[app.route] === undefined) {
    viewless.push(app.route)
    continue
  }
  const shade = (focused(obs) as { shade?: Shade })?.shade
  const vector = shade === undefined ? undefined : buffer(handle, `${shade.route ?? shade.program}/uniforms`)
  if (shade !== undefined && (programs[shade.program] === undefined || vector === undefined || vector.length < 12)) {
    misshaded.push(app.route)
  }
  try {
    const markup = paint(app.route, focused(obs))
    if (markup === "") broken.push(app.route)
    else checkBox(app.route, markup)
    if (shade !== undefined && tally(markup, "canvas") === 0) unforwarded.push(app.route)
  } catch (err) {
    broken.push(`${app.route}: ${String(err)}`)
  }
}
check("every installed app has a view", viewless.length === 0, viewless.join(", "))
check("every view renders its boot state", broken.length === 0, broken.join(", "))
check("every published shade resolves a wasm program", misshaded.length === 0, misshaded.join(", "))
check("every published shade rides its canvas", unforwarded.length === 0, unforwarded.join(", "))
const carved = buffer(handle, "six/tris")
check("the six tris buffer arrives on the wire", carved !== undefined && carved[0] === carved.length - 1 && (carved.length - 1) % 10 === 0, String(carved?.length))
check("every view keeps its altitude and boxes its controls", boxless.length === 0, boxless.join(" | "))

const unbound: string[] = []
const keyed: string[] = []
for (const app of registry.apps) {
  if (app.keys === undefined) continue
  keyed.push(app.route)
  const offered = registry.verbs.find(v => v.app === app.route)?.verbs ?? []
  for (const [dir, hit] of Object.entries(app.keys)) {
    if (!offered.some(v => v.verb === hit.verb)) unbound.push(`${app.route}.${dir}: ${hit.verb}`)
  }
}
check("every bound key names a verb its app offers", unbound.length === 0, unbound.join(", "))

const dropped: string[] = []
for (const group of registry.verbs) {
  for (const verb of group.verbs) {
    if (!plays(verb)) dropped.push(verb.verb)
  }
}
check("no app advertises a verb the roller drops", dropped.length === 0, dropped.join(", "))
const arcade = ["snake", "twenty48", "escape", "tennis", "crush"]
check("the arcade inherits its keys from the manifests", arcade.every(route => keyed.includes(route)), keyed.join(" "))
const crushKeys = registry.apps.find(a => a.route === "crush")?.keys
check(
  "crush binds up to the crush and down to the drop",
  crushKeys?.up?.verb === "crush.crush" && crushKeys.down?.verb === "crush.drop",
  `${crushKeys?.up?.verb} ${crushKeys?.down?.verb}`,
)

const stale: string[] = []
for (const name of readdirSync(new URL("./fixtures", import.meta.url)).sort()) {
  if (!name.endsWith(".json")) continue
  const fixed = JSON.parse(readFileSync(new URL(`./fixtures/${name}`, import.meta.url), "utf8")) as Observation
  const view = fixed.view
  if (view === null || views[view.app] === undefined) continue
  try {
    const markup = paint(view.app, view.state)
    if (markup === "") stale.push(`${name}:${view.app} empty`)
    else {
      const faults = law(markup)
      if (faults.length > 0) stale.push(`${name}:${view.app} ${faults.join(", ")}`)
    }
  } catch (err) {
    stale.push(`${name}:${view.app} ${String(err)}`)
  }
}
check("every fixture state renders through its view", stale.length === 0, stale.join(" | "))

// LOADOUTS

const trimmed = boot("arcade")
const shelved = list(trimmed).apps.map(app => app.route)
const coco = ["notes", "calculator", "calendar", "clock", "timer", "photos", "studio", "colors", "emojis", "fonts", "paint", "dice", "hash"]
check(
  "the arcade boots the system and the games and nothing else",
  shelved.length === 29 && shelved.length + coco.length === registry.apps.length,
  String(shelved.length),
)
check(
  "the arcade leads with the menu and keeps its games",
  shelved[0] === "menu" && shelved.includes("snake") && shelved.includes("solids"),
  shelved.join(" "),
)
check("no coco app rides the arcade", !coco.some(route => shelved.includes(route)), coco.filter(route => shelved.includes(route)).join(" "))
const trespass = call(trimmed, { verb: "nav.open", args: { app: "notes" }, now: now + 1000 })
check("the arcade refuses a route it never booted", trespass.last?.ok === false && trespass.view?.app === "menu", String(trespass.last?.note))

// TIME

const STAMP = now

type Frozen = { route: unknown; apps: unknown }

function lived(world: Handle, calls: Call[]): Frozen {
  for (const made of calls) call(world, { ...made, now: STAMP })
  const iced = call(world, { verb: "sys.freeze", args: {}, now: STAMP }).last?.data as Frozen
  return { route: iced.route, apps: iced.apps }
}

function replayed(calls: Call[]): Frozen {
  const world = boot("full")
  const iced = lived(world, calls)
  world.free()
  return iced
}

function same(a: Frozen, b: Frozen): boolean {
  return JSON.stringify(a) === JSON.stringify(b)
}

const parted: string[] = []
for (const verb of Object.keys(RUNS)) {
  const door: Call = { verb: "nav.open", args: { app: verb.slice(0, verb.indexOf(".")) } }
  const whole = replayed([door, { verb, args: { n: 40 } }])
  const split = replayed([door, { verb, args: { n: 17 } }, { verb, args: { n: 23 } }])
  if (!same(whole, split)) parted.push(verb)
}
check("a run splits where you stand", parted.length === 0, parted.join(", "))

const SCRIPT: Call[] = [
  { verb: "nav.open", args: { app: "snake" } },
  { verb: "snake.reset", args: { seed: 7 } },
  { verb: "snake.turn", args: { dir: "left" } },
  { verb: "snake.step", args: { n: 4 } },
  { verb: "nav.open", args: { app: "life" } },
  { verb: "life.step", args: { n: 6 } },
  { verb: "nav.open", args: { app: "calculator" } },
  { verb: "calculator.digit", args: { d: 6 } },
]
const film: Journal = { iden: "gate", version: registry.version, calls: SCRIPT }

const walked = boot("full")
const marks = new Map<number, Frozen>()
let mark = 0
marks.set(0, lived(walked, []))
for (const made of SCRIPT) {
  mark += stops({ ...film, calls: [made] })
  marks.set(mark, lived(walked, [made]))
}
const drifted: string[] = []
for (const [stop, seen] of marks) {
  if (!same(replayed(upto(film, stop)), seen)) drifted.push(String(stop))
}
check("a life replays to the stop it names", drifted.length === 0, drifted.join(", "))

const cut = upto(film, 6)
check(
  "the journal counts every stop",
  stops(film) === 16 &&
    upto(film, 0).length === 0 &&
    JSON.stringify(upto(film, 16)) === JSON.stringify(SCRIPT) &&
    cut.length === 4 &&
    (cut[3]?.args as { n?: unknown }).n === 3,
  `${stops(film)} stops, cut ${JSON.stringify(cut[3]?.args)}`,
)

const before = marks.get(mark)
const after = lived(walked, [{ verb: "nav.open", args: { app: "menu" } }])
walked.free()
check("turning the head moves no piece", JSON.stringify(before?.apps) === JSON.stringify(after.apps))

console.log(failures === 0 ? "verify green" : `verify red: ${failures} failing`)
if (failures > 0) process.exit(1)
