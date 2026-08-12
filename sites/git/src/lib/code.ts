import { createCssVariablesTheme, createHighlighterCore } from "shiki/core"
import { createJavaScriptRegexEngine } from "shiki/engine/javascript"
import type { HighlighterCore, ShikiTransformer } from "shiki/core"
import { LANGS, PLAIN, lang } from "./langs"

// LANGS

const GRAMMARS: Record<string, () => Promise<unknown>> = {
  c: () => import("@shikijs/langs/c"),
  css: () => import("@shikijs/langs/css"),
  csv: () => import("@shikijs/langs/csv"),
  html: () => import("@shikijs/langs/html"),
  javascript: () => import("@shikijs/langs/javascript"),
  json: () => import("@shikijs/langs/json"),
  jsx: () => import("@shikijs/langs/jsx"),
  markdown: () => import("@shikijs/langs/markdown"),
  python: () => import("@shikijs/langs/python"),
  rust: () => import("@shikijs/langs/rust"),
  shellscript: () => import("@shikijs/langs/shellscript"),
  toml: () => import("@shikijs/langs/toml"),
  tsx: () => import("@shikijs/langs/tsx"),
  typescript: () => import("@shikijs/langs/typescript"),
  wgsl: () => import("@shikijs/langs/wgsl"),
  yaml: () => import("@shikijs/langs/yaml"),
}

export function named(hint: string): string {
  const key = hint.toLowerCase()
  if (key === PLAIN || key === "text" || key === "") return PLAIN
  if (GRAMMARS[key] !== undefined) return key
  return LANGS[key] ?? PLAIN
}

// THEME

const NAME = "css-variables"

const THEME = createCssVariablesTheme({ name: NAME, variablePrefix: "--code-" })

// HIGHLIGHT

let held: Promise<HighlighterCore> | undefined

const loading = new Map<string, Promise<void>>()

function core(): Promise<HighlighterCore> {
  held ??= createHighlighterCore({
    langs: [],
    themes: [THEME],
    engine: createJavaScriptRegexEngine({ forgiving: true }),
  })
  return held
}

async function ready(name: string): Promise<string> {
  const grammar = GRAMMARS[name]
  if (grammar === undefined) return PLAIN
  let job = loading.get(name)
  if (job === undefined) {
    job = core().then(async hl => {
      await hl.loadLanguage((await grammar()) as never)
    })
    loading.set(name, job)
  }
  try {
    await job
  } catch {
    loading.delete(name)
    return PLAIN
  }
  return name
}

const TOKEN = /var\(--code-token-([a-z-]+)\)/

const FACES: [string, string][] = [
  ["font-style:italic", "tk-italic"],
  ["font-weight:bold", "tk-bold"],
  ["text-decoration:underline", "tk-underline"],
]

function marks(style: string): string {
  const out: string[] = []
  const token = TOKEN.exec(style)
  if (token !== null) out.push(`tk-${token[1] ?? ""}`)
  for (const [needle, cls] of FACES) if (style.replace(/\s/g, "").includes(needle)) out.push(cls)
  return out.join(" ")
}

function dress(name: string, digits: number): ShikiTransformer {
  return {
    name: "git-code",
    pre(node) {
      node.properties = { class: `src d${String(digits)}`, tabindex: "0", dataLanguage: name }
    },
    code(node) {
      node.properties = {}
    },
    span(node) {
      const style = node.properties["style"]
      delete node.properties["style"]
      if (typeof style !== "string") return
      const cls = marks(style)
      if (cls === "") delete node.properties["class"]
      else node.properties["class"] = cls
    },
  }
}

const WIDEST = 6

const NARROWEST = 2

function digits(body: string): number {
  const count = String(body.split("\n").length).length
  return Math.min(WIDEST, Math.max(NARROWEST, count))
}

export async function highlight(src: string, hint: string): Promise<string> {
  const body = src.replace(/(?:\r\n|\r|\n)$/, "")
  const name = await ready(named(hint))
  const hl = await core()
  return hl.codeToHtml(body, {
    lang: name,
    theme: NAME,
    transformers: [dress(name, digits(body))],
  })
}

export async function source(path: string, src: string): Promise<string> {
  return highlight(src, lang(path))
}
