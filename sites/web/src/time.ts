import { useSyncExternalStore } from "react"

export type Time = { at: number; span: number }

let time: Time = { at: 0, span: 0 }
const subs = new Set<() => void>()

export function setTime(at: number, span: number): void {
  if (at === time.at && span === time.span) return
  time = { at, span }
  for (const sub of subs) sub()
}

export function useTime(): Time {
  return useSyncExternalStore(
    sub => {
      subs.add(sub)
      return () => subs.delete(sub)
    },
    () => time,
    () => time,
  )
}
