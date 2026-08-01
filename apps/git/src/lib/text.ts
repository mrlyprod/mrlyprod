// BLOCKS

const OPENS = /^(#{1,6} |[-*+] |\d+\. |> |\||```|!\[)/

export function heading(md: string): string {
  for (const raw of md.split("\n")) {
    const line = raw.trimEnd()
    if (line.startsWith("# ")) return line.slice(2)
  }
  return ""
}

export function firstPara(md: string): string {
  const lines: string[] = []
  let fenced = false
  for (const raw of md.split("\n")) {
    const line = raw.trimEnd()
    if (line.startsWith("```")) {
      if (lines.length > 0) break
      fenced = !fenced
      continue
    }
    if (fenced) continue
    const breaks = line === "" || OPENS.test(line)
    if (breaks && lines.length > 0) break
    if (breaks) continue
    lines.push(line)
  }
  return plain(lines.join(" "))
}

// INLINE

export function plain(text: string): string {
  return text.replace(/\[([^\]]*)\]\([^)]*\)/g, "$1").replace(/\*/g, "")
}

// ESCAPE

export function esc(text: string): string {
  return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;")
}
