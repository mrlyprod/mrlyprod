import type { Palette } from "./types.ts"

let store: Palette | null = null

export function install(next: Palette): void {
  store = next
  const root = document.documentElement
  for (const name of next.names) root.style.setProperty(`--c-${name}`, next.hex[name] as string)
}

export function names(): string[] {
  return store?.names ?? []
}

export function pool(): string[] {
  return names().filter(name => name !== "black" && name !== "white")
}

export function hex(name: string): string {
  return (store?.hex[name] as string | undefined) ?? ""
}

export function board(dark: boolean): string {
  const canvas = store?.canvas
  if (canvas === undefined) return ""
  return dark ? canvas.dark : canvas.light
}
