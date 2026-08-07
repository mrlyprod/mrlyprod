import { useSyncExternalStore } from "react"

let shown = true
const subs = new Set<() => void>()

export function setSplash(next: boolean): void {
  if (next === shown) return
  shown = next
  for (const sub of subs) sub()
}

export function useSplash(): boolean {
  return useSyncExternalStore(
    sub => {
      subs.add(sub)
      return () => subs.delete(sub)
    },
    () => shown,
    () => true,
  )
}
