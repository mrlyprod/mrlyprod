import init, { act as rawAct, boot, describe as rawDescribe, frame as rawFrame, geometry as rawGeometry, goose as rawGoose, html as rawHtml, mark as rawMark, palette as rawPalette, peek as rawPeek, shaders as rawShaders, uniforms as rawUniforms } from "../../../pkgs/mrlyjs/web/pkg/mrlyjs.js"
import type { Handle, InitInput } from "../../../pkgs/mrlyjs/web/pkg/mrlyjs.js"
import type { Call, Mark, Observation, Palette, Registry, Shaders, View } from "./types.ts"

export { boot }
export type { Handle }

export async function load(module_or_path: InitInput | Promise<InitInput>): Promise<void> {
  await init({ module_or_path })
}

function pack(shape?: unknown): string | undefined {
  return shape === undefined ? undefined : JSON.stringify(shape)
}

export function describe(shape?: unknown): Registry {
  return JSON.parse(rawDescribe(pack(shape))) as Registry
}

export function shaders(): Shaders {
  return JSON.parse(rawShaders()) as Shaders
}

export function palette(): Palette {
  return JSON.parse(rawPalette()) as Palette
}

export function html(md: string): string {
  return rawHtml(md)
}

export function mark(): Mark {
  return JSON.parse(rawMark()) as Mark
}

export function act(handle: Handle, call: Call): Observation {
  return JSON.parse(rawAct(handle, JSON.stringify(call))) as Observation
}

export function frame(handle: Handle, shape?: unknown): Observation {
  return JSON.parse(rawFrame(handle, pack(shape))) as Observation
}

export function goose(handle: Handle, seed: number): Observation {
  return JSON.parse(rawGoose(handle, BigInt(seed))) as Observation
}

export function peek(handle: Handle, app: string, shape?: unknown): View | null {
  return JSON.parse(rawPeek(handle, app, pack(shape))) as View | null
}

export function geometry(handle: Handle, app: string): Float32Array | undefined {
  return rawGeometry(handle, app)
}

export function uniforms(handle: Handle, app: string): Float32Array | undefined {
  return rawUniforms(handle, app)
}
