import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "react"
import { read, write } from "./lib"

export const DOCK = "(min-width: 74rem)"

export const PANE_MIN = 32

export const PANE_MAX = 4096

export const PANE_WIDE = 260

export type Side = "left" | "right"

type Keys = { width: string; open: string; css: string }

const KEYS: Record<Side, Keys> = {
  left: { width: "mrly-left-width", open: "mrly-left-open", css: "--left-w" },
  right: { width: "mrly-right-width", open: "mrly-right-open", css: "--right-w" },
}

export function clampPane(px: number): number {
  return Math.min(PANE_MAX, Math.max(PANE_MIN, Math.round(px)))
}

export function paintPane(side: Side, px: number): void {
  const root = document.documentElement
  if (px > 0) root.style.setProperty(KEYS[side].css, `${String(px)}px`)
  else root.style.removeProperty(KEYS[side].css)
}

export type Pane = {
  open: boolean
  width: number
  hold: (node: HTMLElement | null) => void
  node: () => HTMLElement | null
  toggle: () => void
  resize: (px: number) => void
  reset: () => void
}

export type PaneSet = { wide: boolean; left: Pane; right: Pane }

export function useWide(): boolean {
  const [wide, set] = useState(() => matchMedia(DOCK).matches)
  useEffect(() => {
    const query = matchMedia(DOCK)
    const sync = (): void => set(query.matches)
    query.addEventListener("change", sync)
    sync()
    return () => query.removeEventListener("change", sync)
  }, [])
  return wide
}

function usePane(side: Side, wide: boolean): Pane {
  const keys = KEYS[side]
  const held = useRef<HTMLElement | null>(null)
  const [width, setWidth] = useState(() => {
    const stored = Number(read(keys.width))
    return stored > 0 ? clampPane(stored) : 0
  })
  const [open, setOpen] = useState(() => matchMedia(DOCK).matches && read(keys.open) !== "0")

  useLayoutEffect(() => {
    paintPane(side, width)
  }, [side, width])

  useEffect(() => {
    setOpen(wide && read(keys.open) !== "0")
  }, [wide, keys.open])

  const hold = useCallback((node: HTMLElement | null): void => {
    held.current = node
  }, [])

  const node = useCallback((): HTMLElement | null => held.current, [])

  const toggle = useCallback((): void => {
    setOpen(was => {
      const next = !was
      if (matchMedia(DOCK).matches) write(keys.open, next ? "" : "0")
      return next
    })
  }, [keys.open])

  const resize = useCallback(
    (px: number): void => {
      const next = clampPane(px)
      setWidth(next)
      write(keys.width, String(next))
    },
    [keys.width],
  )

  const reset = useCallback((): void => {
    setWidth(0)
    write(keys.width, "")
  }, [keys.width])

  return { open, width, hold, node, toggle, resize, reset }
}

export function usePanes(): PaneSet {
  const wide = useWide()
  return { wide, left: usePane("left", wide), right: usePane("right", wide) }
}
