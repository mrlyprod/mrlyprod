import katex from "katex"
import rehypeKatex from "rehype-katex"
import remarkMath from "remark-math"
import type { Options } from "rehype-katex"

// PLUGINS

export { rehypeKatex, remarkMath }

export const KATEX: Options = { strict: false }

// RENDER

export function tex(src: string, block = false): string {
  return katex.renderToString(src, { ...KATEX, displayMode: block, throwOnError: false })
}
