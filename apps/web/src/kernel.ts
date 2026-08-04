import init, { boot, call as rawCall, list as rawList, read as rawRead } from "../../../pkgs/mrlyjs/web/pkg/mrlyjs.js"
import type { Handle, InitInput } from "../../../pkgs/mrlyjs/web/pkg/mrlyjs.js"
import type { Call, Observation, Registry, View } from "./types.ts"

export { boot }
export type { Handle }

export async function load(module_or_path: InitInput | Promise<InitInput>): Promise<void> {
  await init({ module_or_path })
}

function pack(shape?: unknown): string | undefined {
  return shape === undefined ? undefined : JSON.stringify(shape)
}

export function call(handle: Handle, made: Call): Observation {
  return JSON.parse(rawCall(handle, JSON.stringify(made))) as Observation
}

export function list(handle: Handle, shape?: unknown): Registry {
  return JSON.parse(rawList(handle, pack(shape))) as Registry
}

export function read(handle: Handle, path: string, shape?: unknown): unknown {
  return JSON.parse(rawRead(handle, path, pack(shape)))
}

export function observe(handle: Handle): Observation {
  return read(handle, "") as Observation
}

export function view(handle: Handle, app: string): View | null {
  return read(handle, app) as View | null
}

export function buffer(handle: Handle, path: string): Float32Array | undefined {
  const packed = read(handle, path) as { dtype: string; data: string } | null
  if (packed === null || packed.dtype !== "f32") return undefined
  const bytes = Uint8Array.from(atob(packed.data), c => c.charCodeAt(0))
  return new Float32Array(bytes.buffer)
}
