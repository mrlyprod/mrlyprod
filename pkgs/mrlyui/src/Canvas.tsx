import { useCallback, useEffect, useRef } from "react"
import type { CSSProperties, Ref } from "react"
import { cx } from "./lib"

// STEER

export type Steer = { yaw: number; pitch: number; dist: number; panx: number; pany: number }

export type Point = [number, number]

const TAU = Math.PI * 2

const WHEEL = 250

type Grip = { pointers: Map<number, Point>; turn: Point; pan: Point; zoom: number; panning: boolean }

type Hold = {
  grid: Point | undefined
  onTap: ((x: number, y: number) => void) | undefined
  onDrag: ((points: Point[]) => void) | undefined
  onTurn: ((dyaw: number, dpitch: number) => void) | undefined
  onPan: ((dx: number, dy: number) => void) | undefined
  onZoom: ((dir: "in" | "out", n: number) => void) | undefined
  onSteer: ((steer: Steer | null) => void) | undefined
}

// CANVAS

export function Canvas({ grid, paint, sig, crisp, className, style, ref, onTap, onDrag, onTurn, onPan, onZoom, onSteer }: {
  grid?: Point
  paint?: (surface: HTMLCanvasElement) => void
  sig?: string
  crisp?: boolean
  className?: string
  style?: CSSProperties
  ref?: Ref<HTMLCanvasElement>
  onTap?: (x: number, y: number) => void
  onDrag?: (points: Point[]) => void
  onTurn?: (dyaw: number, dpitch: number) => void
  onPan?: (dx: number, dy: number) => void
  onZoom?: (dir: "in" | "out", n: number) => void
  onSteer?: (steer: Steer | null) => void
}) {
  const own = useRef<HTMLCanvasElement | null>(null)
  const hold = useRef<Hold>({ grid, onTap, onDrag, onTurn, onPan, onZoom, onSteer })
  hold.current = { grid, onTap, onDrag, onTurn, onPan, onZoom, onSteer }

  const attach = useCallback(
    (node: HTMLCanvasElement | null) => {
      own.current = node
      if (typeof ref === "function") ref(node)
      else if (ref) ref.current = node
    },
    [ref],
  )

  useEffect(() => {
    const el = own.current
    if (el === null) return

    const cell = (event: PointerEvent): Point | null => {
      const shape = hold.current.grid
      if (shape === undefined) return null
      const [cols, rows] = shape
      if (cols === 0 || rows === 0) return null
      const rect = el.getBoundingClientRect()
      if (rect.width === 0 || rect.height === 0) return null
      const x = Math.floor(((event.clientX - rect.left) / rect.width) * cols)
      const y = Math.floor(((event.clientY - rect.top) / rect.height) * rows)
      return [Math.max(0, Math.min(cols - 1, x)), Math.max(0, Math.min(rows - 1, y))]
    }

    let trail: Point[] | null = null
    let grip: Grip | null = null
    let wheeling: ReturnType<typeof setTimeout> | null = null

    const steered = (): void => {
      if (grip === null) return
      hold.current.onSteer?.({
        yaw: (grip.turn[0] * TAU) / 256,
        pitch: (grip.turn[1] * TAU) / 256,
        dist: grip.zoom / 4,
        panx: grip.pan[0] / 16,
        pany: grip.pan[1] / 16,
      })
    }

    const settle = (): void => {
      if (grip === null) return
      const dyaw = Math.round(grip.turn[0])
      const dpitch = Math.round(grip.turn[1])
      const dx = Math.round(grip.pan[0])
      const dy = Math.round(grip.pan[1])
      const dz = Math.round(grip.zoom)
      grip = null
      hold.current.onSteer?.(null)
      if (dyaw !== 0 || dpitch !== 0) hold.current.onTurn?.(dyaw, dpitch)
      if (dx !== 0 || dy !== 0) hold.current.onPan?.(dx, dy)
      if (dz !== 0) hold.current.onZoom?.(dz < 0 ? "in" : "out", Math.abs(dz))
    }

    const down = (event: PointerEvent): void => {
      const { onTap, onDrag, onTurn, onPan, onZoom } = hold.current
      if (onTap !== undefined || onDrag !== undefined) {
        const at = cell(event)
        if (at !== null) {
          el.setPointerCapture(event.pointerId)
          trail = [at]
        }
      }
      if (onTurn === undefined && onPan === undefined && onZoom === undefined) return
      el.setPointerCapture(event.pointerId)
      grip ??= { pointers: new Map(), turn: [0, 0], pan: [0, 0], zoom: 0, panning: event.shiftKey }
      grip.pointers.set(event.pointerId, [event.clientX, event.clientY])
    }

    const move = (event: PointerEvent): void => {
      const { onDrag, onTurn, onPan, onZoom } = hold.current
      if (trail !== null && onDrag !== undefined) {
        const at = cell(event)
        const last = trail[trail.length - 1]
        if (at !== null && (last === undefined || at[0] !== last[0] || at[1] !== last[1])) trail.push(at)
      }
      if (grip === null) return
      const prev = grip.pointers.get(event.pointerId)
      if (prev === undefined) return
      const rect = el.getBoundingClientRect()
      if (rect.width === 0) return
      const now: Point = [event.clientX, event.clientY]
      if (grip.pointers.size === 2) {
        const other = [...grip.pointers.entries()].find(([id]) => id !== event.pointerId)
        if (other !== undefined) {
          const o = other[1]
          const before = Math.hypot(prev[0] - o[0], prev[1] - o[1])
          const after = Math.hypot(now[0] - o[0], now[1] - o[1])
          if (onZoom !== undefined) grip.zoom += ((before - after) / rect.width) * 16
          if (onPan !== undefined) {
            grip.pan[0] += ((now[0] - prev[0]) / 2 / rect.width) * 40
            grip.pan[1] -= ((now[1] - prev[1]) / 2 / rect.width) * 40
          }
        }
      } else if (grip.panning && onPan !== undefined) {
        grip.pan[0] += ((now[0] - prev[0]) / rect.width) * 40
        grip.pan[1] -= ((now[1] - prev[1]) / rect.width) * 40
      } else if (onTurn !== undefined) {
        grip.turn[0] += ((now[0] - prev[0]) / rect.width) * 128
        grip.turn[1] -= ((now[1] - prev[1]) / rect.width) * 128
      }
      grip.pointers.set(event.pointerId, now)
      steered()
    }

    const up = (event: PointerEvent): void => {
      if (trail !== null) {
        const { onTap, onDrag } = hold.current
        const gesture = trail
        trail = null
        if (onDrag !== undefined && (gesture.length > 1 || onTap === undefined)) onDrag(gesture)
        else if (onTap !== undefined) {
          const first = gesture[0]
          if (first !== undefined) onTap(first[0], first[1])
        }
      }
      if (grip === null) return
      grip.pointers.delete(event.pointerId)
      if (grip.pointers.size === 0) settle()
    }

    const cancel = (event: PointerEvent): void => {
      trail = null
      if (grip === null) return
      grip.pointers.delete(event.pointerId)
      if (grip.pointers.size === 0) settle()
    }

    const wheel = (event: WheelEvent): void => {
      if (hold.current.onZoom === undefined) return
      event.preventDefault()
      grip ??= { pointers: new Map(), turn: [0, 0], pan: [0, 0], zoom: 0, panning: false }
      grip.zoom += event.deltaY / 100
      steered()
      if (wheeling !== null) clearTimeout(wheeling)
      wheeling = setTimeout(() => {
        wheeling = null
        if (grip !== null && grip.pointers.size === 0) settle()
      }, WHEEL)
    }

    el.addEventListener("pointerdown", down)
    el.addEventListener("pointermove", move)
    el.addEventListener("pointerup", up)
    el.addEventListener("pointercancel", cancel)
    el.addEventListener("wheel", wheel, { passive: false })
    return () => {
      if (wheeling !== null) clearTimeout(wheeling)
      el.removeEventListener("pointerdown", down)
      el.removeEventListener("pointermove", move)
      el.removeEventListener("pointerup", up)
      el.removeEventListener("pointercancel", cancel)
      el.removeEventListener("wheel", wheel)
    }
  }, [])

  useEffect(() => {
    const el = own.current
    if (el === null || paint === undefined) return
    paint(el)
  }, [paint, sig])

  const gestured = onTap !== undefined || onDrag !== undefined || onTurn !== undefined || onPan !== undefined || onZoom !== undefined
  const shape: CSSProperties = { ...style }
  if (grid !== undefined) shape.aspectRatio = `${Math.max(grid[0], 1)} / ${Math.max(grid[1], 1)}`
  if (gestured) shape.touchAction = "none"

  return <canvas ref={attach} className={cx("canvas", crisp && "crisp", className)} style={shape} />
}
