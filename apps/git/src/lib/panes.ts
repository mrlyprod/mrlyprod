import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "react"
import { read, write } from "mrlydom"

// SIZES

export const DOCK = "(min-width: 74rem)"

export const MIN = 160

export const MAX = 448

export const WIDE = 240

export type Side = "left" | "right"

type Keys = { width: string; open: string; css: string }

const KEYS: Record<Side, Keys> = {
  left: { width: "mrly-left-width", open: "mrly-left-open", css: "--left-w" },
  right: { width: "mrly-right-width", open: "mrly-right-open", css: "--right-w" },
}

export function clamp(px: number): number {
  return Math.min(MAX, Math.max(MIN, Math.round(px)))
}

export function paint(side: Side, px: number): void {
  const root = document.documentElement
  if (px > 0) root.style.setProperty(KEYS[side].css, `${String(px)}px`)
  else root.style.removeProperty(KEYS[side].css)
}

// SHAPE

export type Pane = {
  open: boolean
  width: number
  hold: (node: HTMLElement | null) => void
  node: () => HTMLElement | null
  toggle: () => void
  resize: (px: number) => void
  reset: () => void
}

// WIDE

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

// PANE

function usePane(side: Side, wide: boolean): Pane {
  const keys = KEYS[side]
  const held = useRef<HTMLElement | null>(null)
  const [width, setWidth] = useState(() => {
    const stored = Number(read(keys.width))
    return stored > 0 ? clamp(stored) : 0
  })
  const [open, setOpen] = useState(() => matchMedia(DOCK).matches && read(keys.open) !== "0")

  useLayoutEffect(() => {
    paint(side, width)
  }, [side, width])

  useEffect(() => {
    setOpen(wide && read(keys.open) !== "0")
  }, [wide, keys.open])

  useEffect(() => {
    const node = held.current
    if (node === null || typeof node.hidePopover !== "function") return
    const shown = node.matches(":popover-open")
    if (wide || !open) {
      if (shown) node.hidePopover()
      return
    }
    if (!shown) node.showPopover()
  }, [wide, open])

  useEffect(() => {
    const node = held.current
    if (node === null) return
    const sync = (event: Event): void => setOpen((event as ToggleEvent).newState === "open")
    node.addEventListener("toggle", sync)
    return () => node.removeEventListener("toggle", sync)
  }, [])

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
      const next = clamp(px)
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

export function usePanes(): { wide: boolean; left: Pane; right: Pane } {
  const wide = useWide()
  return { wide, left: usePane("left", wide), right: usePane("right", wide) }
}
