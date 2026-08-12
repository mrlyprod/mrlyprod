import { lastSegment } from "./repo"

// LANGS

export const PLAIN = "plaintext"

export const LANGS: Record<string, string> = {
  c: "c",
  cjs: "javascript",
  css: "css",
  h: "c",
  html: "html",
  js: "javascript",
  json: "json",
  jsx: "jsx",
  md: "markdown",
  mjs: "javascript",
  py: "python",
  pyi: "python",
  rs: "rust",
  sh: "shellscript",
  bash: "shellscript",
  zsh: "shellscript",
  toml: "toml",
  ts: "typescript",
  tsx: "tsx",
  wgsl: "wgsl",
  yaml: "yaml",
  yml: "yaml",
  csv: "csv",
}

export function lang(path: string): string {
  const name = lastSegment(path)
  const cut = name.lastIndexOf(".")
  if (cut === -1) return PLAIN
  return LANGS[name.slice(cut + 1).toLowerCase()] ?? PLAIN
}
