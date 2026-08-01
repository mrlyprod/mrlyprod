import { unified } from "unified"
import type { Plugin } from "unified"
import remarkParse from "remark-parse"
import remarkGfm from "remark-gfm"
import remarkRehype from "remark-rehype"
import rehypeRaw from "rehype-raw"
import rehypeStringify from "rehype-stringify"
import type { Element, Root } from "hast"
import type { Section } from "mrlydom"

// SHAPE

export type Miss = { route: string; value: string }

export type Doc = { html: string; toc: Section[]; misses: Miss[] }

type Parent = Root | Element

// WALK

function walk(parent: Parent, visit: (node: Element) => void): void {
  for (const node of parent.children) {
    if (node.type !== "element") continue
    visit(node)
    walk(node, visit)
  }
}

function flatten(node: Parent): string {
  let out = ""
  for (const kid of node.children) {
    if (kid.type === "text") out += kid.value
    else if (kid.type === "element") out += flatten(kid)
  }
  return out
}

// SLUGS

const HEADS = ["h1", "h2", "h3", "h4", "h5", "h6"]

function slug(text: string): string {
  return text
    .trim()
    .toLowerCase()
    .replace(/[^\p{L}\p{N}\p{M}\-_ ]/gu, "")
    .replace(/ /g, "-")
}

const heads: Plugin<[Section[]], Root> = function (toc) {
  return tree => {
    const seen = new Map<string, number>()
    walk(tree, node => {
      if (!HEADS.includes(node.tagName)) return
      const text = flatten(node)
      const held = node.properties["id"]
      let id = typeof held === "string" ? held : slug(text)
      const count = seen.get(id) ?? 0
      seen.set(id, count + 1)
      if (count > 0) id = `${id}-${count}`
      node.properties["id"] = id
      if (node.tagName === "h2") toc.push({ id, text })
    })
  }
}

// REFS

const OUTSIDE = ["mailto:", "/", "#"]

function routeOf(href: string, routes: string[]): string | undefined {
  const name = href.split("/").pop() ?? ""
  const stem = (name.endsWith(".md") ? name.slice(0, -3) : name).toLowerCase()
  return routes.find(route => route.split("/").pop() === stem)
}

const refs: Plugin<[string, string[], Miss[]], Root> = function (from, routes, misses) {
  return tree => {
    walk(tree, node => {
      if (node.tagName !== "a") return
      const href = node.properties["href"]
      if (typeof href !== "string" || href === "") return
      if (href.startsWith("http://") || href.startsWith("https://")) {
        node.properties["target"] = "_blank"
        node.properties["rel"] = ["noopener"]
        return
      }
      if (OUTSIDE.some(mark => href.startsWith(mark))) return
      const route = routeOf(href, routes)
      if (route === undefined) misses.push({ route: from, value: href })
      else node.properties["href"] = `/${route}`
    })
  }
}

// RENDER

export async function render(route: string, body: string, routes: string[]): Promise<Doc> {
  const toc: Section[] = []
  const misses: Miss[] = []
  const file = await unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkRehype, { allowDangerousHtml: true })
    .use(rehypeRaw)
    .use(heads, toc)
    .use(refs, route, routes, misses)
    .use(rehypeStringify, { allowDangerousHtml: true })
    .process(body)
  return { html: String(file), toc, misses }
}

// CHECK

export async function scan(route: string, body: string, routes: string[]): Promise<Miss[]> {
  const { misses } = await render(route, body, routes)
  return misses
}
