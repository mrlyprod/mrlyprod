import { useSyncExternalStore } from "react"
import { call } from "../builders"
import type { Call } from "../types"

export type Orbit = { yaw: number; pitch: number; dist: number; pan: [number, number]; ortho: boolean }

const TAU = Math.PI * 2
const TURN = 256
const RIGGED = new Set(["bang", "solids", "three"])
const rigs = new Map<string, Orbit>()
const subs = new Set<() => void>()

let nonce = 0

const clamp = (value: number, lo: number, hi: number): number => Math.max(lo, Math.min(hi, value))

// RIG

export function orbit(app: string): Orbit {
  let rig = rigs.get(app)
  if (rig === undefined) {
    rig = { yaw: 32, pitch: 20, dist: 12, pan: [0, 0], ortho: app === "three" }
    rigs.set(app, rig)
  }
  return rig
}

export function aim(app: string, u: Float32Array | undefined): Float32Array | undefined {
  if (u === undefined || !RIGGED.has(app)) return u
  const rig = orbit(app)
  u[13] = (rig.yaw * TAU) / TURN
  u[14] = (rig.pitch * TAU) / TURN
  u[15] = rig.dist / 4
  u[16] = rig.pan[0] / 16
  u[17] = rig.pan[1] / 16
  u[18] = rig.ortho ? 1 : 0
  return u
}

// SPIN

export function spin(made: Call): boolean {
  if (!made.verb.startsWith("orbit.")) return false
  const app = String(made.args.app ?? "")
  if (!RIGGED.has(app)) return true
  const rig = orbit(app)
  const num = (key: string): number => Number(made.args[key] ?? 0)
  if (made.verb === "orbit.turn") {
    rig.yaw = (((rig.yaw + num("dyaw")) % TURN) + TURN) % TURN
    rig.pitch = clamp(rig.pitch + num("dpitch"), -56, 56)
  }
  if (made.verb === "orbit.zoom") {
    const step = made.args.dir === "in" ? -num("n") : num("n")
    rig.dist = clamp(rig.dist + step, 8, 32)
  }
  if (made.verb === "orbit.pan") {
    rig.pan = [clamp(rig.pan[0] + num("dx"), -32, 32), clamp(rig.pan[1] + num("dy"), -32, 32)]
  }
  if (made.verb === "orbit.ortho") rig.ortho = made.args.value === true
  nonce += 1
  for (const sub of subs) sub()
  return true
}

// HOOK

export function useSpin(): number {
  return useSyncExternalStore(
    sub => {
      subs.add(sub)
      return () => subs.delete(sub)
    },
    () => nonce,
    () => nonce,
  )
}

export function useOrbit(app: string): {
  rig: Orbit
  turn: Call
  pan: Call
  zoom: Call
  onOrtho: (value: boolean) => void
} {
  useSpin()
  return {
    rig: orbit(app),
    turn: call("orbit.turn", { app }),
    pan: call("orbit.pan", { app }),
    zoom: call("orbit.zoom", { app }),
    onOrtho: value => {
      spin(call("orbit.ortho", { app, value }))
    },
  }
}
