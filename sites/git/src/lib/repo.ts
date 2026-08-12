import { ROOT } from "./site"

// SHAPE

export type Entry = { path: string; doc: boolean }

export type Dir = {
  readme: string | undefined
  subdirs: string[]
  files: [string, string][]
  assets: [string, string][]
}

export type Ctx = {
  tracked: Set<string>
  dirs: Map<string, Dir>
  files: Map<string, Entry>
  updated: Map<string, string>
  meta: Map<string, [string, string]>
}

// MANIFEST

export type Node = {
  r?: 1
  u?: string
  d?: string[]
  f?: [string, string][]
  a?: string[]
}

export type Manifest = {
  dirs: Record<string, Node>
  meta: Record<string, [string, string]>
}

export const README = "README.md"

// PATHS

export function split(path: string): [string, string] {
  const cut = path.lastIndexOf("/")
  return cut === -1 ? ["", path] : [path.slice(0, cut), path.slice(cut + 1)]
}

export function join(dir: string, seg: string): string {
  return dir === "" ? seg : `${dir}/${seg}`
}

export function lastSegment(route: string): string {
  return split(route)[1]
}

export function fileRoute(path: string): string {
  const [dir, name] = split(path)
  const stem = (name.endsWith(".md") ? name.slice(0, -3) : name).toLowerCase()
  return join(dir, stem)
}

export function dirTitle(route: string): string {
  return route === "" ? ROOT : lastSegment(route)
}

// BUILD

export function fromManifest(manifest: Manifest): Ctx {
  const tracked = new Set<string>()
  const dirs = new Map<string, Dir>()
  const files = new Map<string, Entry>()
  const updated = new Map<string, string>()
  for (const [route, node] of Object.entries(manifest.dirs)) {
    const readme = node.r === undefined ? undefined : join(route, README)
    const named: [string, string][] = []
    const assets: [string, string][] = []
    for (const [name, date] of node.f ?? []) {
      const path = join(route, name)
      const target = fileRoute(path)
      tracked.add(path)
      files.set(target, { path, doc: path.endsWith(".md") })
      if (date !== "") updated.set(target, date)
      named.push([name, target])
    }
    for (const name of node.a ?? []) {
      const path = join(route, name)
      tracked.add(path)
      assets.push([name, path])
    }
    if (readme !== undefined) {
      tracked.add(readme)
      if (node.u !== undefined) updated.set(route, node.u)
    }
    dirs.set(route, { readme, subdirs: node.d ?? [], files: named, assets })
  }
  const meta = new Map(Object.entries(manifest.meta) as [string, [string, string]][])
  return { tracked, dirs, files, updated, meta }
}

// ROUTES

function order(a: string, b: string): number {
  return a < b ? -1 : a > b ? 1 : 0
}

export function routes(ctx: Ctx): string[] {
  return [...new Set([...ctx.dirs.keys(), ...ctx.files.keys()])].sort(order)
}

export function routeDate(ctx: Ctx, route: string): string | undefined {
  return ctx.updated.get(route)
}

export function sourcePath(ctx: Ctx, route: string): string | undefined {
  const file = ctx.files.get(route)
  if (file !== undefined) return file.path
  return ctx.dirs.get(route)?.readme
}

// TITLES

export function title(ctx: Ctx, route: string): string {
  const held = ctx.meta.get(route)
  if (held !== undefined) return held[0]
  const file = ctx.files.get(route)
  if (file !== undefined) return lastSegment(file.path)
  return dirTitle(route)
}

export function desc(ctx: Ctx, route: string): string {
  const held = ctx.meta.get(route)
  if (held !== undefined) return held[1]
  const file = ctx.files.get(route)
  if (file !== undefined) return file.path
  return dirTitle(route)
}

// RESOLVE

function normalize(slug: string, from: string): string[] | undefined {
  const parts = from.split("/").filter(s => s !== "")
  for (const seg of slug.split("/")) {
    if (seg === "" || seg === ".") continue
    if (seg === "..") {
      if (parts.pop() === undefined) return undefined
      continue
    }
    parts.push(seg)
  }
  return parts
}

export function resolveAsset(ctx: Ctx, slug: string, from: string): string | undefined {
  const parts = normalize(slug, from)
  if (parts === undefined) return undefined
  const joined = parts.join("/")
  return ctx.tracked.has(joined) ? joined : undefined
}

export function resolve(ctx: Ctx, slug: string, from: string): string | undefined {
  const parts = normalize(slug, from)
  if (parts === undefined) return undefined
  const joined = parts.join("/")
  if (ctx.dirs.has(joined)) return joined
  const doc = `${joined}.md`
  if (ctx.tracked.has(doc)) {
    const [dir, name] = split(doc)
    return name === README ? dir : fileRoute(doc)
  }
  const route = joined.toLowerCase()
  return ctx.files.has(route) ? route : undefined
}
