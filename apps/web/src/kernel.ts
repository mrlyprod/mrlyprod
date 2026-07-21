import init, { act as rawAct, boot, describe as rawDescribe, designs as rawDesigns, frame as rawFrame, geometry as rawGeometry, glyphs as rawGlyphs, html as rawHtml, mark as rawMark, palette as rawPalette, peek as rawPeek, shaders as rawShaders } from "../../../pkgs/mrlyjs/pkg/mrlyjs.js"
import type { Handle, InitInput } from "../../../pkgs/mrlyjs/pkg/mrlyjs.js"
import type { Call, Designs, DesignsReq, GlyphSet, Mark, Observation, Palette, Registry, Shaders, View } from "./types.ts"

export { boot }
export type { Handle }

export async function load(module_or_path: InitInput | Promise<InitInput>): Promise<void> {
  await init({ module_or_path })
}

export function describe(): Registry {
  return JSON.parse(rawDescribe()) as Registry
}

export function shaders(): Shaders {
  return JSON.parse(rawShaders()) as Shaders
}

export function palette(): Palette {
  return JSON.parse(rawPalette()) as Palette
}

export function glyphs(set: string): GlyphSet[] {
  return JSON.parse(rawGlyphs(set)) as GlyphSet[]
}

export function designs(req: DesignsReq): Designs {
  return JSON.parse(rawDesigns(JSON.stringify(req))) as Designs
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

export function frame(handle: Handle): Observation {
  return JSON.parse(rawFrame(handle)) as Observation
}

export function peek(handle: Handle, app: string): View | null {
  return JSON.parse(rawPeek(handle, app)) as View | null
}

export function geometry(handle: Handle, app: string): Float32Array | undefined {
  return rawGeometry(handle, app)
}
