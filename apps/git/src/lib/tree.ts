import type { Ctx } from "./repo"
import { join, lastSegment } from "./repo"
import { RAW, ROOT } from "./site"
import { esc } from "./text"

// KINDS

const KINDS: Record<string, string> = {
  ".gitignore": "git",
  ".prettierignore": "config",
  ".prettierrc": "json",
  ".python-version": "python",
  license: "license",
  astro: "html",
  c: "c",
  css: "css",
  csv: "csv",
  h: "h",
  html: "html",
  ico: "favicon",
  js: "js",
  json: "json",
  lock: "lock",
  md: "markdown",
  mjs: "js",
  png: "image",
  py: "python",
  rs: "rust",
  sh: "shell",
  svg: "svg",
  toml: "config",
  ts: "ts",
  tsx: "react",
  ttf: "font",
  wgsl: "cuda",
  woff: "font",
  woff2: "font",
}

export function kind(name: string): string {
  const low = name.toLowerCase()
  const dot = low.lastIndexOf(".")
  const key = KINDS[low] ?? (dot > 0 ? KINDS[low.slice(dot + 1)] : undefined)
  return key === undefined ? "" : ` si-${key}`
}

// ROWS

const NONE = " "

type Row = [string, string, boolean]

const shut = new Map<string, string>()

function byName(a: Row, b: Row): number {
  const x = a[0].toLowerCase()
  const y = b[0].toLowerCase()
  return x < y ? -1 : x > y ? 1 : 0
}

function leaf(href: string, name: string, cur: string): string {
  return `<a class="leaf" href="${href}"${cur}><span class="si${kind(name)}" aria-hidden="true"></span>${esc(name)}</a>\n`
}

function level(ctx: Ctx, route: string, current: string): string {
  const d = ctx.dirs.get(route)
  if (d === undefined) return ""
  let h = ""
  for (const sub of d.subdirs) {
    const target = join(route, sub)
    const on = current === target || current.startsWith(`${target}/`)
    const cur = current === target ? ' aria-current="page"' : ""
    h += `<details${on ? " open" : ""}><summary><span class="ic mark" aria-hidden="true">chevron_right</span>`
    h += `<a href="/${target}"${cur}>${esc(sub)}</a></summary>\n<div class="sub">\n`
    h += on ? level(ctx, target, current) : closed(ctx, target)
    h += "</div>\n</details>\n"
  }
  const rows: Row[] = []
  for (const [name, target] of d.files) rows.push([name, `/${target}`, current === target])
  for (const [name, path] of d.assets) rows.push([name, `${RAW}${path}`, false])
  if (d.readme !== undefined) rows.push([lastSegment(d.readme), `/${route}`, false])
  for (const [name, href, live] of rows.sort(byName)) {
    h += leaf(href, name, live ? ' aria-current="page"' : "")
  }
  return h
}

function closed(ctx: Ctx, route: string): string {
  let h = shut.get(route)
  if (h === undefined) {
    h = level(ctx, route, NONE)
    shut.set(route, h)
  }
  return h
}

// EXPLORER

export function explorer(ctx: Ctx, current: string): string {
  const root = current === "" ? ' aria-current="page"' : ""
  return `<a class="root" href="/"${root}>${ROOT}</a>\n${level(ctx, "", current)}`
}
