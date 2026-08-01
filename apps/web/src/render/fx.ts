type Scrambling = HTMLElement & { __scramble?: number }

const CHARS = "abcdef0123456789"

const SCRAMBLE = 300

export function scramble(el: HTMLElement, text: string): void {
  const host = el as Scrambling
  if (host.__scramble !== undefined) cancelAnimationFrame(host.__scramble)
  const chars = Array.from(text)
  const start = performance.now()
  const step = (now: number) => {
    const shown = Math.min(chars.length, Math.ceil(((now - start) / SCRAMBLE) * chars.length))
    el.textContent = chars.map((c, i) => (i < shown || c === " " ? c : CHARS.charAt(Math.floor(Math.random() * CHARS.length)))).join("")
    host.__scramble = shown < chars.length ? requestAnimationFrame(step) : undefined
  }
  host.__scramble = requestAnimationFrame(step)
}
