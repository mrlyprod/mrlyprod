import { useEffect, useRef } from "react"
import type { KeyboardEvent, PointerEvent } from "react"
import type { Pane, Side } from "../lib/panes"
import { MAX, MIN, WIDE, clamp, paint } from "../lib/panes"

// GRIP

const LABELS: Record<Side, string> = { left: "resize explorer", right: "resize contents" }

const STEP = 16

const LEAP = 48

export function Grip({ side, pane }: { side: Side; pane: Pane }) {
  const held = useRef(pane.width)

  useEffect(() => {
    return () => document.documentElement.classList.remove("dragging")
  }, [])

  const down = (event: PointerEvent<HTMLDivElement>): void => {
    const node = pane.node()
    if (node === null || event.button !== 0) return
    const grip = event.currentTarget
    const root = document.documentElement
    const from = event.clientX
    const start = node.getBoundingClientRect().width
    const move = (moved: globalThis.PointerEvent): void => {
      const step = side === "left" ? moved.clientX - from : from - moved.clientX
      held.current = clamp(start + step)
      paint(side, held.current)
    }
    const up = (): void => {
      grip.removeEventListener("pointermove", move)
      grip.removeEventListener("pointerup", up)
      grip.removeEventListener("pointercancel", up)
      root.classList.remove("dragging")
      pane.resize(held.current)
    }
    event.preventDefault()
    grip.setPointerCapture(event.pointerId)
    root.classList.add("dragging")
    held.current = clamp(start)
    grip.addEventListener("pointermove", move)
    grip.addEventListener("pointerup", up)
    grip.addEventListener("pointercancel", up)
  }

  const press = (event: KeyboardEvent<HTMLDivElement>): void => {
    const now = pane.width || WIDE
    const step = event.shiftKey ? LEAP : STEP
    const grow = side === "left" ? "ArrowRight" : "ArrowLeft"
    const shrink = side === "left" ? "ArrowLeft" : "ArrowRight"
    if (event.key === grow) pane.resize(now + step)
    else if (event.key === shrink) pane.resize(now - step)
    else if (event.key === "Home" || event.key === "Enter") pane.reset()
    else return
    event.preventDefault()
  }

  return (
    <div
      className={`grip ${side}`}
      role="separator"
      tabIndex={0}
      aria-orientation="vertical"
      aria-label={LABELS[side]}
      aria-valuenow={pane.width || WIDE}
      aria-valuemin={MIN}
      aria-valuemax={MAX}
      onPointerDown={down}
      onDoubleClick={pane.reset}
      onKeyDown={press}
    />
  )
}
