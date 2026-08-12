import { readdirSync, readFileSync } from "node:fs"
import { join } from "node:path"
import type { Meta, Site } from "../src/lib/data"
import { pathOf } from "../src/lib/data"
import { firstPara, heading } from "../src/lib/text"

// SHAPE

export type Marked = { route: string; title: string; desc: string; md: string }

export type Scan = { site: Site; docs: Marked[] }

// ORDER

const PRODUCTS = ["math", "bricks", "sheets", "sculptures"]

const FRONT = ["research", "blog"]

const FINE = ["about", "contact", "privacy", "terms"]

function posts(base: string): string[] {
  return readdirSync(join(base, "blog"))
    .filter(name => name.endsWith(".md") && name !== "README.md")
    .sort()
    .map(name => `blog/${name.slice(0, -3)}`)
}

// SCAN

function marked(base: string, route: string): Marked {
  const md = readFileSync(join(base, pathOf(route)), "utf8")
  const title = heading(md) || route
  return { route, title, desc: firstPara(md) || title, md }
}

function metas(docs: Marked[]): Record<string, Meta> {
  const out: Record<string, Meta> = {}
  for (const doc of docs) out[doc.route] = [doc.title, doc.desc]
  return out
}

export function scan(base: string): Scan {
  const home = marked(base, "")
  const products = PRODUCTS.map(route => marked(base, route))
  const pages = [...FRONT, ...posts(base), ...FINE].map(route => marked(base, route))
  return {
    site: { home: [home.title, home.desc], products: metas(products), pages: metas(pages) },
    docs: [...products, ...pages],
  }
}
