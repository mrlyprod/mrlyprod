import { unified } from "unified"
import type { Plugin } from "unified"
import remarkParse from "remark-parse"
import remarkGfm from "remark-gfm"
import remarkRehype from "remark-rehype"
import rehypeRaw from "rehype-raw"
import rehypeStringify from "rehype-stringify"
import type { Element, Root, RootContent } from "hast"
import { highlight } from "./code"
import type { Ctx } from "./repo"
import { resolve, resolveAsset, split } from "./repo"
import { RAW } from "./site"

// SHAPE

export type Anchor = { id: string; text: string }

export type Miss = { path: string; kind: "link" | "image"; value: string }

export type Doc = { html: string; toc: Anchor[]; misses: Miss[] }

type Parent = Root | Element

// WALK

function walk(parent: Parent, visit: (node: Element, parent: Parent, index: number) => void): void {
  const kids = parent.children
  for (let i = 0; i < kids.length; i += 1) {
    const node = kids[i]
    if (node === undefined || node.type !== "element") continue
    visit(node, parent, i)
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

function classes(node: Element): string[] {
  const held: unknown = node.properties["className"]
  if (Array.isArray(held)) return held.map(String)
  return typeof held === "string" ? held.split(" ") : []
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

const heads: Plugin<[Anchor[]], Root> = function (toc) {
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

const OUTSIDE = ["mailto:", "#"]

const refs: Plugin<[Ctx, string, Miss[]], Root> = function (ctx, path, misses) {
  const from = split(path)[0]
  return tree => {
    walk(tree, node => {
      if (node.tagName !== "a" && node.tagName !== "img") return
      const value = node.properties["href"] ?? node.properties["src"]
      if (typeof value !== "string" || value === "") return
      if (value.startsWith("http://") || value.startsWith("https://")) {
        if (node.tagName === "a") {
          node.properties["target"] = "_blank"
          node.properties["rel"] = ["noopener"]
        }
        return
      }
      if (OUTSIDE.some(mark => value.startsWith(mark))) return
      if (node.tagName === "a") {
        const stem = (value.startsWith("./") ? value.slice(2) : value).replace(/\.md$/, "")
        const route = resolve(ctx, stem, from)
        if (route === undefined) misses.push({ path, kind: "link", value })
        else node.properties["href"] = `/${route}`
        return
      }
      const asset = resolveAsset(ctx, value, from)
      if (asset === undefined) {
        misses.push({ path, kind: "image", value })
        return
      }
      node.properties["src"] = `${RAW}${asset}`
      node.properties["loading"] = "lazy"
      node.properties["decoding"] = "async"
    })
  }
}

// TABLES

const tables: Plugin<[], Root> = function () {
  return tree => {
    walk(tree, (node, parent, index) => {
      if (node.tagName !== "table") return
      parent.children[index] = {
        type: "element",
        tagName: "div",
        properties: { className: ["table-wrap"] },
        children: [node],
      }
    })
  }
}

// BARE

const bare: Plugin<[], Root> = function () {
  return tree => {
    walk(tree, node => {
      delete node.properties["style"]
    })
  }
}

// CODE

const fences: Plugin<[], Root> = function () {
  return async tree => {
    const found: [Parent, number, string, string][] = []
    walk(tree, (node, parent, index) => {
      if (node.tagName !== "pre") return
      const code = node.children.find(kid => kid.type === "element" && kid.tagName === "code")
      if (code === undefined || code.type !== "element") return
      const hint = classes(code).find(name => name.startsWith("language-"))
      found.push([parent, index, hint === undefined ? "" : hint.slice(9), flatten(code)])
    })
    for (const [parent, index, hint, src] of found) {
      const html = await highlight(src, hint)
      parent.children[index] = { type: "raw", value: html } as unknown as RootContent
    }
  }
}

// RENDER

export async function render(ctx: Ctx, path: string, body: string): Promise<Doc> {
  const toc: Anchor[] = []
  const misses: Miss[] = []
  const file = await unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkRehype, { allowDangerousHtml: true })
    .use(rehypeRaw)
    .use(heads, toc)
    .use(refs, ctx, path, misses)
    .use(tables)
    .use(bare)
    .use(fences)
    .use(rehypeStringify, { allowDangerousHtml: true })
    .process(body)
  return { html: String(file), toc, misses }
}

// CHECK

export async function scan(ctx: Ctx, path: string, body: string): Promise<Miss[]> {
  const misses: Miss[] = []
  await unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkRehype, { allowDangerousHtml: true })
    .use(rehypeRaw)
    .use(refs, ctx, path, misses)
    .use(rehypeStringify, { allowDangerousHtml: true })
    .process(body)
  return misses
}
