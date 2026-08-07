import type { TreeNode } from "mrlyui"
import type { Ctx } from "./repo"
import { join, lastSegment } from "./repo"
import { RAW } from "./site"

// PATHS

export function href(route: string): string {
  return route === "" ? "/" : `/${route}`
}

// ROWS

function byName(a: TreeNode, b: TreeNode): number {
  const x = a.name.toLowerCase()
  const y = b.name.toLowerCase()
  return x < y ? -1 : x > y ? 1 : 0
}

export function leaves(ctx: Ctx, route: string): TreeNode[] {
  const dir = ctx.dirs.get(route)
  if (dir === undefined) return []
  const rows: TreeNode[] = []
  for (const [name, target] of dir.files) rows.push({ name, href: href(target) })
  for (const [name, path] of dir.assets) rows.push({ name, href: `${RAW}${path}` })
  if (dir.readme !== undefined) rows.push({ name: lastSegment(dir.readme), href: href(route) })
  return rows.sort(byName)
}

// EXPLORER

function level(ctx: Ctx, route: string): TreeNode[] {
  const dir = ctx.dirs.get(route)
  if (dir === undefined) return []
  const nodes: TreeNode[] = []
  for (const sub of dir.subdirs) {
    const target = join(route, sub)
    nodes.push({ name: sub, href: href(target), nodes: level(ctx, target) })
  }
  return [...nodes, ...leaves(ctx, route)]
}

export function explorer(ctx: Ctx): TreeNode[] {
  return level(ctx, "")
}
