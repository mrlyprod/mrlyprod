import { spawnSync } from "node:child_process"
import { readFileSync } from "node:fs"
import { join as pathJoin } from "node:path"
import type { Ctx, Dir, Entry, Manifest, Node } from "../src/lib/repo"
import { README, dirTitle, fileRoute, join, lastSegment, split } from "../src/lib/repo"
import { firstPara, heading } from "../src/lib/text"

// SHAPE

const ASSETS = ["png", "jpg", "jpeg", "gif", "webp", "ico", "svg", "woff", "woff2", "ttf", "wasm", "pdf"]

export type Scan = {
  base: string
  ctx: Ctx
  paths: string[]
  manifest: Manifest
  text: (path: string) => string
}

type Draft = {
  readme: string | undefined
  subdirs: Set<string>
  files: Map<string, string>
  assets: Set<string>
}

// GIT

function git(base: string, args: string[]): string {
  const out = spawnSync("git", args, { cwd: base, encoding: "utf8", maxBuffer: 1 << 30 })
  if (out.status !== 0) throw new Error(`git (${args.join(" ")} failed)`)
  return out.stdout
}

export function toplevel(): string {
  const out = spawnSync("git", ["rev-parse", "--show-toplevel"], { encoding: "utf8" })
  if (out.status !== 0) throw new Error("git (rev-parse failed)")
  return out.stdout.trim()
}

function tracked(base: string): string[] {
  return git(base, ["ls-files", "-z"])
    .split("\0")
    .filter(path => path !== "")
}

export function updatedMap(log: string): Map<string, string> {
  const map = new Map<string, string>()
  let date = ""
  for (const line of log.split("\n")) {
    if (line.startsWith("\0")) date = line.slice(1)
    else if (line !== "" && date !== "" && !map.has(line)) map.set(line, date)
  }
  return map
}

// READ

function reader(base: string): [(path: string) => string, (path: string) => boolean] {
  const held = new Map<string, string>()
  const text = (path: string): string => {
    let body = held.get(path)
    if (body === undefined) {
      body = readFileSync(pathJoin(base, path), "utf8")
      held.set(path, body)
    }
    return body
  }
  const readable = (path: string): boolean => {
    const name = lastSegment(path)
    const ext = name.includes(".") ? (name.split(".").pop() ?? "").toLowerCase() : ""
    if (ASSETS.includes(ext)) return false
    try {
      const bytes = readFileSync(pathJoin(base, path))
      held.set(path, new TextDecoder("utf-8", { fatal: true }).decode(bytes))
      return true
    } catch {
      return false
    }
  }
  return [text, readable]
}

// ORGANIZE

function order(a: string, b: string): number {
  return a < b ? -1 : a > b ? 1 : 0
}

function draft(dirs: Map<string, Draft>, key: string): Draft {
  let d = dirs.get(key)
  if (d === undefined) {
    d = { readme: undefined, subdirs: new Set(), files: new Map(), assets: new Set() }
    dirs.set(key, d)
  }
  return d
}

function organize(paths: string[], readable: (path: string) => boolean): [Map<string, Dir>, Map<string, Entry>, Set<string>] {
  const trail = new Set<string>()
  const drafts = new Map<string, Draft>()
  const files = new Map<string, Entry>()
  for (const path of paths) {
    trail.add(path)
    const [dir, name] = split(path)
    draft(drafts, "")
    let acc = ""
    for (const seg of dir.split("/").filter(s => s !== "")) {
      const parent = acc
      acc = join(acc, seg)
      draft(drafts, parent).subdirs.add(seg)
      draft(drafts, acc)
    }
    const doc = path.endsWith(".md")
    if (doc && name === README) {
      draft(drafts, dir).readme = path
      continue
    }
    if (!doc && !readable(path)) {
      draft(drafts, dir).assets.add(name)
      continue
    }
    const route = fileRoute(path)
    if (files.has(route)) throw new Error(`git (route collision at /${route})`)
    files.set(route, { path, doc })
    draft(drafts, dir).files.set(name, route)
  }
  for (const route of files.keys()) {
    if (drafts.has(route)) throw new Error(`git (route collision at /${route})`)
  }
  const dirs = new Map<string, Dir>()
  for (const key of [...drafts.keys()].sort(order)) {
    const d = drafts.get(key) as Draft
    dirs.set(key, {
      readme: d.readme,
      subdirs: [...d.subdirs].sort(order),
      files: [...d.files].sort((a, b) => order(a[0], b[0])),
      assets: [...d.assets].sort(order).map(name => [name, join(key, name)] as [string, string]),
    })
  }
  const sorted = new Map<string, Entry>()
  for (const key of [...files.keys()].sort(order)) sorted.set(key, files.get(key) as Entry)
  return [dirs, sorted, trail]
}

// CARDS

function cards(dirs: Map<string, Dir>, files: Map<string, Entry>, text: (path: string) => string): Map<string, [string, string]> {
  const meta = new Map<string, [string, string]>()
  const card = (route: string, source: string, fallback: string): void => {
    const body = text(source)
    const head = heading(body) || fallback
    meta.set(route, [head, firstPara(body) || head])
  }
  for (const [route, dir] of dirs) {
    if (dir.readme !== undefined) card(route, dir.readme, dirTitle(route))
  }
  for (const [route, file] of files) {
    if (!file.doc) continue
    const name = lastSegment(file.path)
    card(route, file.path, name.slice(0, -3))
  }
  return meta
}

// EMIT

function nodes(dirs: Map<string, Dir>, stamps: Map<string, string>): Record<string, Node> {
  const out: Record<string, Node> = {}
  for (const [route, dir] of dirs) {
    const node: Node = {}
    if (dir.readme !== undefined) {
      node.r = 1
      const date = stamps.get(dir.readme)
      if (date !== undefined) node.u = date
    }
    if (dir.subdirs.length > 0) node.d = dir.subdirs
    if (dir.files.length > 0) node.f = dir.files.map(([name]) => [name, stamps.get(join(route, name)) ?? ""])
    if (dir.assets.length > 0) node.a = dir.assets.map(([name]) => name)
    out[route] = node
  }
  return out
}

// SCAN

export function scan(): Scan {
  const base = toplevel()
  const paths = tracked(base)
  const [text, readable] = reader(base)
  const [dirs, files, trail] = organize(paths, readable)
  const stamps = updatedMap(git(base, ["-c", "core.quotepath=off", "log", "--name-only", "--format=%x00%cs"]))
  const updated = new Map<string, string>()
  for (const [route, file] of files) {
    const date = stamps.get(file.path)
    if (date !== undefined) updated.set(route, date)
  }
  for (const [route, dir] of dirs) {
    if (dir.readme === undefined) continue
    const date = stamps.get(dir.readme)
    if (date !== undefined) updated.set(route, date)
  }
  const meta = cards(dirs, files, text)
  const ctx: Ctx = { tracked: trail, dirs, files, updated, meta }
  const manifest: Manifest = { dirs: nodes(dirs, stamps), meta: Object.fromEntries(meta) }
  return { base, ctx, paths, manifest, text }
}
