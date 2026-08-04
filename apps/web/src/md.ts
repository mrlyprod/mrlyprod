import type { Element, Root } from "hast"
import rehypeStringify from "rehype-stringify"
import remarkGfm from "remark-gfm"
import remarkParse from "remark-parse"
import remarkRehype from "remark-rehype"
import { unified } from "unified"

const SAFE = /^[A-Za-z0-9/\-_.]+$/

function walk(node: Root | Element, fn: (el: Element) => void): void {
  for (const child of node.children) {
    if (child.type === "element") {
      fn(child)
      walk(child, fn)
    }
  }
}

function resolve(tree: Root): Root {
  walk(tree, el => {
    if (el.tagName === "a") {
      const href = String(el.properties.href ?? "")
      if (href.startsWith("http://") || href.startsWith("https://")) {
        el.properties.target = "_blank"
        el.properties.rel = ["noopener"]
        return
      }
      delete el.properties.href
      const held = href.startsWith("./") ? href.slice(2) : href
      const slug = held.endsWith(".md") ? held.slice(0, -3) : held
      if (SAFE.test(slug)) el.properties.dataSlug = slug
    }
    if (el.tagName === "img") {
      const src = String(el.properties.src ?? "")
      delete el.properties.src
      if (src !== "" && SAFE.test(src)) el.properties.dataSlug = src
    }
  })
  return tree
}

const pipe = unified().use(remarkParse).use(remarkGfm).use(remarkRehype).use(() => resolve).use(rehypeStringify)

export function html(md: string): string {
  return String(pipe.processSync(md))
}
