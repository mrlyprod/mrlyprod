import * as gpu from "mrlygpu"
import type { Router } from "../router.ts"
import { entries } from "./boards.ts"

export type Orbit = { yaw: number; pitch: number; dist: number; pan: [number, number]; ortho: boolean }

const TAU = Math.PI * 2
const TURN = 256
const RIGGED = new Set(["bang", "solids", "three"])
const rigs = new Map<string, Orbit>()

const clamp = (value: number, lo: number, hi: number): number => Math.max(lo, Math.min(hi, value))

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

export function install(routes: Router): void {
  routes.on("orbit.", call => {
    const app = String(call.args.app ?? "")
    if (!RIGGED.has(app)) return
    const rig = orbit(app)
    const num = (key: string): number => Number(call.args[key] ?? 0)
    if (call.verb === "orbit.turn") {
      rig.yaw = (((rig.yaw + num("dyaw")) % TURN) + TURN) % TURN
      rig.pitch = clamp(rig.pitch + num("dpitch"), -56, 56)
    }
    if (call.verb === "orbit.zoom") {
      const step = call.args.dir === "in" ? -num("n") : num("n")
      rig.dist = clamp(rig.dist + step, 8, 32)
    }
    if (call.verb === "orbit.pan") {
      rig.pan = [clamp(rig.pan[0] + num("dx"), -32, 32), clamp(rig.pan[1] + num("dy"), -32, 32)]
    }
    if (call.verb === "orbit.ortho") rig.ortho = call.args.value === true
    repaint(app)
  })
}

function repaint(app: string): void {
  for (const [surface, node] of entries()) {
    if (node.handle !== app || node.shade === undefined || !surface.isConnected) continue
    gpu.draw(surface, node, gpu.pull(node.shade))
  }
}
