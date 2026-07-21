import { readFileSync, readdirSync } from "node:fs"
import { act, boot, describe, frame, load, mark, peek, shaders } from "./src/kernel.ts"
import { install as installPeeks } from "./src/peeks.ts"
import type { Call, Node, Observation, Shade } from "./src/types.ts"
import { views } from "./src/views/index.ts"

const wasm = readFileSync(new URL("../../pkgs/mrlyjs/pkg/mrlyjs_bg.wasm", import.meta.url))
await load(wasm)

const handle = boot()
installPeeks(app => peek(handle, app))
const registry = describe()
let now = 1783600496000
let failures = 0

function check(name: string, ok: boolean, detail = "") {
  if (!ok) failures += 1
  console.log(`${ok ? "ok  " : "FAIL"} ${name}${detail === "" ? "" : ` (${detail})`}`)
}

function send(verb: string, args: Call["args"] = {}): Observation {
  now += 1000
  return act(handle, { verb, args, now })
}

function look(): Observation {
  return frame(handle)
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

function nodes(tree: Node): Node[] {
  const kids = "children" in tree && Array.isArray(tree.children) ? (tree.children as Node[]) : []
  return [tree, ...kids.flatMap(nodes)]
}

function boxLaw(node: Node, boxed: boolean): string[] {
  const label = `${node.kind}:${node.key ?? "?"}`
  if (node.kind === "Overlay") return boxLaw(node.child, false)
  if (node.kind === "Mark") return []
  if (node.kind === "Card") {
    const inside = node.children.flatMap(child => boxLaw(child, true))
    return boxed ? [`card-in-card ${label}`, ...inside] : inside
  }
  if (node.kind === "Stack" || node.kind === "Grid") return node.children.flatMap(child => boxLaw(child, boxed))
  return boxed ? [] : [`naked ${label}`]
}

const boxless: string[] = []

function checkBox(route: string, tree: Node) {
  const violations = boxLaw(tree, false)
  if (violations.length > 0) boxless.push(`${route}: ${violations.join(", ")}`)
}

const booted = frame(handle)
check("boots at menu, tick 0", booted.route?.app === "menu" && booted.tick === 0)
check("registry names a version", typeof registry.version === "string" && registry.version !== "")

const anim = mark()
check(
  "the mark animation rides the wasm",
  anim.rows === 7 && anim.cols === 49 && anim.fps === 25 && anim.frames.length > 100,
  `${anim.frames.length} frames`,
)

const programs = shaders()
check(
  "the shader programs ride the wasm",
  Object.keys(programs).length > 0 && Object.values(programs).every(source => source.includes("fn vs_main") && source.includes("fn fs_main")),
  Object.keys(programs).join(" "),
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
send("settings.set", { key: "render", value: "gpu" })
check("settings.set lands the render", state()["render"] === "gpu")
const webgl = send("settings.set", { key: "render", value: "webgl" })
check("settings.set holds the render options", webgl.last?.ok === false)
send("settings.set", { key: "render", value: "cpu" })
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

visit("piano")
const pressed = send("piano.press", { midi: 43 })
const tone = pressed.effects?.[0] as { kind: string; data: { op?: string; id?: string; wave?: string } } | undefined
check(
  "piano.press holds the key and starts a sound",
  (state()["held"] as number[]).includes(43) && tone?.kind === "sound" && tone.data.op === "start" && tone.data.id === "piano:43",
)
check("the pressed key rings the worn wave", tone?.data.wave === "square", String(tone?.data.wave))
const lifted = send("piano.lift", { midi: 43 })
const hush = lifted.effects?.[0] as { kind: string; data: { op?: string; id?: string } } | undefined
check(
  "piano.lift releases the key and stops the sound",
  (state()["held"] as number[]).length === 0 && hush?.kind === "sound" && hush.data.op === "stop" && hush.data.id === "piano:43",
)
const unheld = send("piano.lift", { midi: 43 })
check("an orphan lift fails honestly", unheld.last?.ok === false)
const keyboard = views["piano"]?.(focused(look()), () => {}) as Node
const pressable = nodes(keyboard).filter(n => n.kind === "Button" && n.press !== undefined && n.lift !== undefined)
check("the piano hangs 21 pressable keys", pressable.length === 21, String(pressable.length))
visit("settings")
send("settings.set", { key: "wave", value: "sine" })

send("settings.set", { key: "font", value: "mrly" })
const worn = visit("calculator")
const glyph = focused(worn)["glyph"] as { text?: string; rows?: number[][] } | undefined
check("the glyph flag rasters the readout", glyph?.text === "42" && glyph.rows?.length === 5, JSON.stringify(glyph?.text))
const readout = views["calculator"]?.(focused(worn), () => {})
check("the raster rides the view as a canvas", readout !== undefined && nodes(readout).some(n => n.kind === "Canvas"))
if (readout !== undefined) checkBox("calculator (glyph)", readout)
visit("settings")
send("settings.set", { key: "font", value: "mono" })
visit("calculator")
check("flag down clears the raster", state()["glyph"] === undefined)

visit("font")
send("font.pick", { char: "a" })
check("font picks a glyph", state()["char"] === "a", String(state()["char"]))
const lit = () => ((state()["glyph"] as { rows: number[][] }).rows.flat().filter(v => v !== 0).length)
const full = lit()
send("font.scramble")
const dark = lit()
send("font.tick")
check("the scramble reveals pixel by pixel", full > 0 && dark === 0 && lit() === 1, `${full} ${dark} ${lit()}`)

const pixeling = visit("pixel")
const pixelTree = views["pixel"]?.(focused(pixeling), () => {}) as Node
const board = nodes(pixelTree).find(n => n.kind === "Canvas") as Extract<Node, { kind: "Canvas" }> | undefined
check("pixel declares the stroke on its canvas", board?.drag?.verb === "pixel.stroke" && board.grid?.[0] === 24)
checkBox("pixel", pixelTree)
send("pixel.stroke", { points: [[0, 0], [1, 1]] })
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
const lifeTree = views["life"]?.(focused(living), () => {}) as Node
const lifeCanvas = nodes(lifeTree).find(n => n.kind === "Canvas") as Extract<Node, { kind: "Canvas" }> | undefined
const settings = () => state()["settings"] as Record<string, unknown>
check("life sizes its canvas to the board", lifeCanvas?.grid?.[0] === settings()["size"])
checkBox("life", lifeTree)
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
const tileLibrary = (peek(handle, "tile")?.state as { library: { value: unknown }[] }).library
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
const tileTree = views["tile"]?.(focused(look()), () => {}) as Node
check("the tile view hangs a preview canvas", nodes(tileTree).some(n => n.kind === "Canvas"))
checkBox("tile", tileTree)

const kept = (app: string) => (peek(handle, app)?.state as { library: unknown[] }).library
check("colors keeps the full name library", kept("colors").length === 15, String(kept("colors").length))
check("emoji keeps a non-empty library", kept("emoji").length > 0, String(kept("emoji").length))
check("font keeps a non-empty library", kept("font").length > 0, String(kept("font").length))
visit("tile")
send("tile.roll")
const saved = send("tile.save")
check("tile.save keeps the current tile", saved.last?.ok === true, String(saved.last?.note))

const PNG =
  "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg=="
visit("photos")
const loaded = send("photos.load")
const effect = loaded.effects?.[0] as { kind: string; data: { url?: string }; call?: { verb: string } } | undefined
check(
  "photos.load emits the seeded fetch effect",
  effect?.kind === "fetch" && effect.call?.verb === "photos.land" && (effect.data.url ?? "").includes("picsum.photos"),
)
send("photos.land", { data: PNG, mime: "image/png" })
const wall = state()["photos"] as string[]
check("landed bytes hang as a data uri", wall.length === 1 && wall[0]?.startsWith("data:image/png;base64,") === true)
const orphan = send("photos.land", { data: PNG })
check("an unrequested land fails honestly", orphan.last?.ok === false)
check("photos declares internet in its manifest", registry.apps.find(a => a.route === "photos")?.internet === true)

check("nav carries the open verb", registry.nav.length === 1 && registry.nav[0]?.verb === "nav.open", registry.nav.map(v => v.verb).join(" "))
const swapped = send("nav.open", { app: "menu" })
check("nav.open replaces the view", swapped.view?.app === "menu" && swapped.route?.app === "menu")
const ghost = send("nav.open", { app: "ghost" })
check("a missing app is refused", ghost.last?.ok === false && ghost.view?.app === "menu", String(ghost.last?.note))

let viewless: string[] = []
let broken: string[] = []
let misshaded: string[] = []
let unforwarded: string[] = []
for (const app of registry.apps) {
  const obs = visit(app.route)
  const draw = views[app.route]
  if (draw === undefined) {
    viewless.push(app.route)
    continue
  }
  const shade = (focused(obs) as { shade?: Shade })?.shade
  if (shade !== undefined && (programs[shade.program] === undefined || !Array.isArray(shade.uniforms) || shade.uniforms.length < 12)) {
    misshaded.push(app.route)
  }
  try {
    const tree = draw(focused(obs), () => {})
    if (nodes(tree).length === 0) broken.push(app.route)
    else checkBox(app.route, tree)
    if (shade !== undefined && !nodes(tree).some(n => n.kind === "Canvas" && n.shade?.program === shade.program)) {
      unforwarded.push(app.route)
    }
  } catch (err) {
    broken.push(`${app.route}: ${String(err)}`)
  }
}
check("every installed app has a view", viewless.length === 0, viewless.join(", "))
check("every view renders its boot state", broken.length === 0, broken.join(", "))
check("every published shade resolves a wasm program", misshaded.length === 0, misshaded.join(", "))
check("every published shade rides its canvas", unforwarded.length === 0, unforwarded.join(", "))
check("every view keeps each node in exactly one border-box", boxless.length === 0, boxless.join(" | "))

const stale: string[] = []
for (const name of readdirSync(new URL("./fixtures", import.meta.url)).sort()) {
  if (!name.endsWith(".json")) continue
  const fixed = JSON.parse(readFileSync(new URL(`./fixtures/${name}`, import.meta.url), "utf8")) as Observation
  const view = fixed.view
  const draw = view === null ? undefined : views[view.app]
  if (view === null || draw === undefined) continue
  try {
    const tree = draw(view.state, () => {})
    if (nodes(tree).length === 0) stale.push(`${name}:${view.app} empty`)
    else {
      const violations = boxLaw(tree, false)
      if (violations.length > 0) stale.push(`${name}:${view.app} ${violations.join(", ")}`)
    }
  } catch (err) {
    stale.push(`${name}:${view.app} ${String(err)}`)
  }
}
check("every fixture state renders through its view", stale.length === 0, stale.join(" | "))

console.log(failures === 0 ? "verify green" : `verify red: ${failures} failing`)
if (failures > 0) process.exit(1)
