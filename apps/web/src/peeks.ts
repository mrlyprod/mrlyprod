import type { View } from "./types.ts"

let store: (app: string) => View | null = () => null

export function install(fn: (app: string) => View | null): void {
  store = fn
}

export function peeked(app: string): View | null {
  return store(app)
}
