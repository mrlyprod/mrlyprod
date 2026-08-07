import { useCallback, useEffect, useRef } from "react"
import * as gpu from "mrlygpu"
import { Canvas } from "mrlyui"
import type { Point, Steer } from "mrlyui"
import { useSend } from "../send"
import { spin, useSpin } from "./orbit"
import { dark } from "./theme"
import type { Args, Call, Shade } from "../types"

export function Shader({ shade, grid, handle, className, turn, pan, zoom }: {
  shade: Shade | undefined
  grid?: Point
  handle?: string
  className?: string
  turn?: Call
  pan?: Call
  zoom?: Call
}) {
  const own = useRef<HTMLCanvasElement | null>(null)
  const send = useSend()
  useSpin()
  const shading = shade === undefined ? null : gpu.pull(shade)
  const held = useRef<{ shade: Shade | undefined; shading: number[] | null }>({ shade, shading })
  held.current = { shade, shading }

  const paint = useCallback((surface: HTMLCanvasElement) => {
    const now = held.current
    if (now.shade === undefined) return
    gpu.draw(surface, { shade: now.shade }, now.shading)
  }, [])

  useEffect(() => {
    const el = own.current
    if (el === null) return
    return () => gpu.drop(el)
  }, [])

  useEffect(() => {
    const el = own.current
    if (el === null || handle === undefined) return
    el.dataset.handle = handle
  }, [handle])

  const steer = (offsets: Steer | null): void => {
    const el = own.current
    if (el !== null) gpu.steer(el, offsets)
  }

  const fire = (made: Call, args: Args): void => {
    const next: Call = { verb: made.verb, args: { ...made.args, ...args } }
    if (spin(next)) return
    send(next)
  }

  const sig = `${dark()}:${shade?.program ?? ""}:${shade?.route ?? ""}:${shade?.mesh ?? ""}:${shading?.join(",") ?? ""}`

  return (
    <Canvas
      ref={own}
      grid={grid}
      paint={paint}
      sig={sig}
      className={className}
      onSteer={steer}
      onTurn={turn === undefined ? undefined : (dyaw, dpitch) => fire(turn, { dyaw, dpitch })}
      onPan={pan === undefined ? undefined : (dx, dy) => fire(pan, { dx, dy })}
      onZoom={zoom === undefined ? undefined : (dir, n) => fire(zoom, { dir, n })}
    />
  )
}
