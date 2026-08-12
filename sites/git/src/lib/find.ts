import type { Ctx } from "./repo"
import { desc, dirTitle, lastSegment, split, title } from "./repo"
import { ROOT } from "./site"

// SHAPE

export type Hit = { route: string; name: string; hint: string }

type Row = { route: string; name: string; hint: string; path: string; head: string; lede: string }

const LIMIT = 60

// INDEX

const held = new WeakMap<Ctx, Row[]>()

function rows(ctx: Ctx): Row[] {
  const out: Row[] = []
  for (const route of ctx.dirs.keys()) {
    out.push({
      route,
      name: dirTitle(route),
      hint: route === "" ? ROOT : route,
      path: route,
      head: title(ctx, route).toLowerCase(),
      lede: desc(ctx, route).toLowerCase(),
    })
  }
  for (const [route, file] of ctx.files) {
    const dir = split(file.path)[0]
    out.push({
      route,
      name: lastSegment(file.path),
      hint: dir === "" ? ROOT : dir,
      path: file.path,
      head: title(ctx, route).toLowerCase(),
      lede: desc(ctx, route).toLowerCase(),
    })
  }
  return out
}

function index(ctx: Ctx): Row[] {
  let all = held.get(ctx)
  if (all === undefined) {
    all = rows(ctx)
    held.set(ctx, all)
  }
  return all
}

// HUNT

function score(row: Row, needle: string): [number, number] | undefined {
  const path = row.path.toLowerCase().indexOf(needle)
  if (path !== -1) return [0, path]
  const head = row.head.indexOf(needle)
  if (head !== -1) return [1, head]
  const lede = row.lede.indexOf(needle)
  if (lede !== -1) return [2, lede]
  return undefined
}

export function hunt(ctx: Ctx, query: string): Hit[] {
  const needle = query.trim().toLowerCase()
  if (needle === "") return []
  const found: [number, number, number, Row][] = []
  for (const row of index(ctx)) {
    const rank = score(row, needle)
    if (rank !== undefined) found.push([rank[0], rank[1], row.path.length, row])
  }
  found.sort((a, b) => a[0] - b[0] || a[1] - b[1] || a[2] - b[2] || (a[3].path < b[3].path ? -1 : 1))
  return found.slice(0, LIMIT).map(([, , , row]) => ({ route: row.route, name: row.name, hint: row.hint }))
}
