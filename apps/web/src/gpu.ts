import { board } from "./palette.ts"
import type { Node, Shade, Shaders } from "./types.ts"

type Board = Extract<Node, { kind: "Canvas" }>

type Program = { pipeline: GPURenderPipeline; textured: boolean }

type Spec = { linear: number[]; angle: number[]; viewport: number[] }

type Tween = { prev: number[]; next: number[]; from: number; span: number; spec: Spec }

type Steer = { yaw: number; pitch: number; dist: number; panx?: number; pany?: number }

type Shape = {
  sig: string
  tris: GPUBuffer | null
  triCount: number
  lines: GPUBuffer | null
  lineCount: number
  depth: GPUTexture | null
}

type MeshPipes = {
  opaque: GPURenderPipeline
  back: GPURenderPipeline
  front: GPURenderPipeline
  lines: GPURenderPipeline
  layout: GPUBindGroupLayout
}

type Slot = {
  context: GPUCanvasContext
  buffer: GPUBuffer | null
  texture: GPUTexture | null
  group: GPUBindGroup | null
  program: string
  node: Board
  watcher: ResizeObserver
  sent: number[] | null
  stamp: number
  tween: Tween | null
  steer: Steer | null
  shape: Shape | null
}

const BEAT = 125
const LINGER = 500
const TAU = Math.PI * 2
const PITCH_LIM = (56 / 256) * TAU
const PAN_LIM = 2

const TWEENS: Record<string, Spec> = {
  mandelbrot: { linear: [2, 11, 18], angle: [], viewport: [12, 13, 14, 15] },
  julia: { linear: [2, 11, 16, 17, 20], angle: [], viewport: [12, 13, 14, 15] },
  mesh: { linear: [15, 16, 17, 19], angle: [12, 13, 14, 20, 21], viewport: [] },
}

const STEER: Record<string, Steer> = {
  mesh: { yaw: 13, pitch: 14, dist: 15, panx: 16, pany: 17 },
}

let sources: Shaders = {}
let fetcher: ((route: string) => Float32Array | undefined) | null = null
let device: GPUDevice | null = null
let format: GPUTextureFormat = "bgra8unorm"
let sampler: GPUSampler | null = null
let meshPipes: MeshPipes | null = null
const programs = new Map<string, Program>()
const slots = new WeakMap<HTMLCanvasElement, Slot>()
const tints = new Map<string, [number, number, number, number]>()

export function init(shaders: Shaders, geometry?: (route: string) => Float32Array | undefined): void {
  sources = shaders
  fetcher = geometry ?? null
  const gpu = navigator.gpu
  if (gpu === undefined) return
  void gpu
    .requestAdapter()
    .then(async adapter => {
      if (adapter === null) return
      const granted = await adapter.requestDevice()
      format = gpu.getPreferredCanvasFormat()
      device = granted
    })
    .catch(() => undefined)
}

function program(name: string): Program | null {
  const found = programs.get(name)
  if (found !== undefined) return found
  const wgsl = sources[name]
  if (wgsl === undefined || device === null) return null
  const module = device.createShaderModule({ code: wgsl })
  const pipeline = device.createRenderPipeline({
    layout: "auto",
    vertex: { module, entryPoint: "vs_main" },
    fragment: { module, entryPoint: "fs_main", targets: [{ format }] },
    primitive: { topology: "triangle-list" },
  })
  const made = { pipeline, textured: wgsl.includes("texture_2d") }
  programs.set(name, made)
  return made
}

function mesh(): MeshPipes | null {
  if (meshPipes !== null) return meshPipes
  const wgsl = sources["mesh"]
  if (wgsl === undefined || device === null) return null
  const module = device.createShaderModule({ code: wgsl })
  const layout = device.createBindGroupLayout({
    entries: [{ binding: 0, visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT, buffer: {} }],
  })
  const pipes = device.createPipelineLayout({ bindGroupLayouts: [layout] })
  const tris: GPUVertexBufferLayout = {
    arrayStride: 24,
    attributes: [
      { shaderLocation: 0, offset: 0, format: "float32x3" },
      { shaderLocation: 1, offset: 12, format: "float32x3" },
    ],
  }
  const lines: GPUVertexBufferLayout = {
    arrayStride: 32,
    attributes: [
      { shaderLocation: 0, offset: 0, format: "float32x3" },
      { shaderLocation: 1, offset: 12, format: "float32" },
      { shaderLocation: 2, offset: 16, format: "float32x4" },
    ],
  }
  const blend: GPUBlendState = {
    color: { srcFactor: "one", dstFactor: "one-minus-src-alpha" },
    alpha: { srcFactor: "one", dstFactor: "one-minus-src-alpha" },
  }
  const depth = (write: boolean): GPUDepthStencilState => ({ format: "depth24plus", depthWriteEnabled: write, depthCompare: "less" })
  const pass = (vertex: string, fragment: string, buffers: GPUVertexBufferLayout, topology: GPUPrimitiveTopology, write: boolean, blended: boolean) =>
    (device as GPUDevice).createRenderPipeline({
      layout: pipes,
      vertex: { module, entryPoint: vertex, buffers: [buffers] },
      fragment: { module, entryPoint: fragment, targets: [blended ? { format, blend } : { format }] },
      primitive: { topology },
      depthStencil: depth(write),
    })
  meshPipes = {
    opaque: pass("vs_main", "fs_main", tris, "triangle-list", true, false),
    back: pass("vs_main", "fs_back", tris, "triangle-list", false, true),
    front: pass("vs_main", "fs_front", tris, "triangle-list", false, true),
    lines: pass("vs_line", "fs_line", lines, "line-list", true, true),
    layout,
  }
  return meshPipes
}

function vertexBuffer(data: Float32Array): GPUBuffer {
  const buf = (device as GPUDevice).createBuffer({ size: data.byteLength, usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST })
  ;(device as GPUDevice).queue.writeBuffer(buf, 0, data)
  return buf
}

function ensureShape(slot: Slot, shade: Shade): Shape | null {
  const held = slot.shape
  if (held !== null && held.sig === shade.mesh) return held
  if (fetcher === null || shade.route === undefined || shade.mesh === undefined) return null
  const packed = fetcher(shade.route)
  if (packed === undefined) return null
  const triFloats = packed[0] ?? 0
  const lineFloats = packed[1] ?? 0
  const triData = packed.subarray(2, 2 + triFloats)
  const lineData = packed.subarray(2 + triFloats, 2 + triFloats + lineFloats)
  held?.tris?.destroy()
  held?.lines?.destroy()
  const made: Shape = {
    sig: shade.mesh,
    tris: triFloats > 0 ? vertexBuffer(triData) : null,
    triCount: triFloats / 6,
    lines: lineFloats > 0 ? vertexBuffer(lineData) : null,
    lineCount: lineFloats / 8,
    depth: held?.depth ?? null,
  }
  slot.shape = made
  return made
}

function ensureDepth(shape: Shape, width: number, height: number): GPUTexture {
  if (shape.depth !== null && shape.depth.width === width && shape.depth.height === height) return shape.depth
  shape.depth?.destroy()
  const depth = (device as GPUDevice).createTexture({
    size: [width, height],
    format: "depth24plus",
    usage: GPUTextureUsage.RENDER_ATTACHMENT,
  })
  shape.depth = depth
  return depth
}

function paintMesh(canvas: HTMLCanvasElement, slot: Slot, shade: Shade): void {
  const pipes = mesh()
  if (pipes === null || device === null) return
  const { width, height } = fit(canvas)
  const data = Float32Array.from(slot.tween !== null ? blend(slot, performance.now()) : shade.uniforms)
  if (slot.steer !== null) bend(data, shade.program, slot.steer)
  data[0] = width
  data[1] = height
  if (slot.buffer === null || slot.buffer.size !== data.byteLength) {
    slot.buffer?.destroy()
    slot.buffer = device.createBuffer({ size: data.byteLength, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST })
    slot.group = null
  }
  device.queue.writeBuffer(slot.buffer, 0, data)
  if (slot.group === null || slot.program !== "mesh") {
    slot.group = device.createBindGroup({ layout: pipes.layout, entries: [{ binding: 0, resource: { buffer: slot.buffer } }] })
    slot.program = "mesh"
  }
  const shape = ensureShape(slot, shade)
  if (shape === null) return
  const depth = ensureDepth(shape, width, height)
  const encoder = device.createCommandEncoder()
  const pass = encoder.beginRenderPass({
    colorAttachments: [{ view: slot.context.getCurrentTexture().createView(), loadOp: "clear", storeOp: "store", clearValue: clearColor() }],
    depthStencilAttachment: { view: depth.createView(), depthClearValue: 1, depthLoadOp: "clear", depthStoreOp: "store" },
  })
  const group = slot.group
  const alpha = data[19] as number
  if (shape.tris !== null && shape.triCount > 0) {
    pass.setBindGroup(0, group)
    pass.setVertexBuffer(0, shape.tris)
    if (alpha >= 1) {
      pass.setPipeline(pipes.opaque)
      pass.draw(shape.triCount)
    } else {
      pass.setPipeline(pipes.back)
      pass.draw(shape.triCount)
      pass.setPipeline(pipes.front)
      pass.draw(shape.triCount)
    }
  }
  if (shape.lines !== null && shape.lineCount > 0) {
    pass.setPipeline(pipes.lines)
    pass.setBindGroup(0, group)
    pass.setVertexBuffer(0, shape.lines)
    pass.draw(shape.lineCount)
  }
  pass.end()
  device.queue.submit([encoder.finish()])
}

export function draw(canvas: HTMLCanvasElement, node: Board): boolean {
  if (device === null || node.shade === undefined) return false
  if (node.shade.program === "mesh" ? mesh() === null : program(node.shade.program) === null) return false
  let slot = slots.get(canvas)
  if (slot === undefined) {
    const context = canvas.getContext("webgpu") as unknown as GPUCanvasContext | null
    if (context === null) return false
    context.configure({ device, format, alphaMode: "premultiplied" })
    const watcher = new ResizeObserver(() => {
      const held = slots.get(canvas)
      if (held !== undefined && canvas.isConnected) paint(canvas, held)
    })
    watcher.observe(canvas)
    slot = { context, buffer: null, texture: null, group: null, program: "", node, watcher, sent: null, stamp: 0, tween: null, steer: null, shape: null }
    slots.set(canvas, slot)
  }
  commit(canvas, slot, node)
  paint(canvas, slot)
  return true
}

const animating = new Set<HTMLCanvasElement>()
let looping = false

const calm = matchMedia("(prefers-reduced-motion: reduce)")
calm.addEventListener("change", () => {
  if (!calm.matches) return
  for (const canvas of animating) {
    const slot = slots.get(canvas)
    if (slot !== undefined) {
      slot.tween = null
      if (canvas.isConnected) paint(canvas, slot)
    }
    animating.delete(canvas)
  }
})

function commit(canvas: HTMLCanvasElement, slot: Slot, node: Board): void {
  const held = slot.node.shade
  slot.node = node
  const shade = node.shade
  if (shade === undefined) return
  const spec = TWEENS[shade.program]
  const now = performance.now()
  const prev = slot.tween !== null ? blend(slot, now) : slot.sent
  const smooth =
    spec !== undefined &&
    !calm.matches &&
    prev !== null &&
    held !== undefined &&
    held.program === shade.program &&
    prev.length === shade.uniforms.length &&
    !jumps(spec, prev, shade.uniforms)
  if (smooth) {
    const span = Math.min(Math.max(now - slot.stamp, BEAT), LINGER)
    slot.tween = { prev: prev as number[], next: shade.uniforms, from: now, span, spec: spec as Spec }
    animating.add(canvas)
    cycle()
  } else {
    slot.tween = null
    animating.delete(canvas)
  }
  slot.sent = shade.uniforms
  slot.stamp = now
}

function bend(data: number[] | Float32Array, program: string, offsets: Steer): void {
  const idx = STEER[program]
  if (idx === undefined) return
  data[idx.yaw] = (data[idx.yaw] as number) + offsets.yaw
  data[idx.pitch] = Math.max(-PITCH_LIM, Math.min(PITCH_LIM, (data[idx.pitch] as number) + offsets.pitch))
  data[idx.dist] = Math.max(2, Math.min(8, (data[idx.dist] as number) + offsets.dist))
  if (idx.panx !== undefined && offsets.panx !== undefined) {
    data[idx.panx] = Math.max(-PAN_LIM, Math.min(PAN_LIM, (data[idx.panx] as number) + offsets.panx))
  }
  if (idx.pany !== undefined && offsets.pany !== undefined) {
    data[idx.pany] = Math.max(-PAN_LIM, Math.min(PAN_LIM, (data[idx.pany] as number) + offsets.pany))
  }
}

export function steer(canvas: HTMLCanvasElement, offsets: Steer | null): boolean {
  const slot = slots.get(canvas)
  if (slot === undefined || device === null) return false
  const shade = slot.node.shade
  if (shade === undefined || STEER[shade.program] === undefined) return false
  if (offsets === null) {
    if (slot.steer !== null && slot.sent !== null) {
      const folded = slot.sent.slice()
      bend(folded, shade.program, slot.steer)
      slot.sent = folded
    }
    slot.steer = null
    return true
  }
  slot.tween = null
  animating.delete(canvas)
  slot.steer = offsets
  if (canvas.isConnected) paint(canvas, slot)
  return true
}

function cycle(): void {
  if (looping) return
  looping = true
  requestAnimationFrame(step)
}

function step(): void {
  const now = performance.now()
  for (const canvas of animating) {
    const slot = slots.get(canvas)
    if (slot === undefined || slot.tween === null || !canvas.isConnected) {
      animating.delete(canvas)
      continue
    }
    if (now - slot.tween.from >= slot.tween.span) {
      slot.tween = null
      animating.delete(canvas)
    }
    paint(canvas, slot)
  }
  if (animating.size === 0) {
    looping = false
    return
  }
  requestAnimationFrame(step)
}

function blend(slot: Slot, now: number): number[] {
  const { prev, next, from, span, spec } = slot.tween as Tween
  const t = Math.min(Math.max((now - from) / span, 0), 1)
  const out = next.slice()
  for (const i of spec.linear) out[i] = (prev[i] as number) + ((next[i] as number) - (prev[i] as number)) * t
  for (const i of spec.angle) {
    const a = prev[i] as number
    let d = (next[i] as number) - a
    d -= Math.round(d / TAU) * TAU
    out[i] = a + d * t
  }
  if (spec.viewport.length === 4) {
    const [xa, xb, ya, yb] = spec.viewport as [number, number, number, number]
    const x = axis(prev[xa] as number, prev[xb] as number, next[xa] as number, next[xb] as number, t)
    const y = axis(prev[ya] as number, prev[yb] as number, next[ya] as number, next[yb] as number, t)
    out[xa] = x[0]
    out[xb] = x[1]
    out[ya] = y[0]
    out[yb] = y[1]
  }
  return out
}

function axis(min0: number, max0: number, min1: number, max1: number, t: number): [number, number] {
  const c0 = (min0 + max0) / 2
  const c1 = (min1 + max1) / 2
  const s0 = Math.max(max0 - min0, 1e-12)
  const s1 = Math.max(max1 - min1, 1e-12)
  const c = c0 + (c1 - c0) * t
  const s = Math.exp(Math.log(s0) + (Math.log(s1) - Math.log(s0)) * t)
  return [c - s / 2, c + s / 2]
}

function jumps(spec: Spec, prev: number[], next: number[]): boolean {
  if (spec.viewport.length !== 4) return false
  const [xa, xb, ya, yb] = spec.viewport as [number, number, number, number]
  return (
    leaps(prev[xa] as number, prev[xb] as number, next[xa] as number, next[xb] as number) ||
    leaps(prev[ya] as number, prev[yb] as number, next[ya] as number, next[yb] as number)
  )
}

function leaps(min0: number, max0: number, min1: number, max1: number): boolean {
  const s0 = Math.max(max0 - min0, 1e-12)
  const s1 = Math.max(max1 - min1, 1e-12)
  const ratio = s1 > s0 ? s1 / s0 : s0 / s1
  if (ratio > 8) return true
  return Math.abs((min1 + max1) / 2 - (min0 + max0) / 2) > s0 + s1
}

function fit(canvas: HTMLCanvasElement): { width: number; height: number } {
  const rect = canvas.getBoundingClientRect()
  const width = Math.max(1, Math.round(rect.width * window.devicePixelRatio))
  const height = Math.max(1, Math.round(rect.height * window.devicePixelRatio))
  if (canvas.width !== width) canvas.width = width
  if (canvas.height !== height) canvas.height = height
  return { width, height }
}

function clearColor(): { r: number; g: number; b: number; a: number } {
  const dark = document.body.classList.contains("darkmode")
  const [br, bg, bb] = tint(board(dark))
  return { r: br / 255, g: bg / 255, b: bb / 255, a: 1 }
}

function paint(canvas: HTMLCanvasElement, slot: Slot): void {
  const shade = slot.node.shade
  if (device === null || shade === undefined) return
  if (shade.program === "mesh") {
    paintMesh(canvas, slot, shade)
    return
  }
  const made = program(shade.program)
  if (made === null) return
  const { width, height } = fit(canvas)
  const data = Float32Array.from(slot.tween !== null ? blend(slot, performance.now()) : shade.uniforms)
  if (slot.steer !== null) bend(data, shade.program, slot.steer)
  data[0] = width
  data[1] = height
  if (slot.buffer === null || slot.buffer.size !== data.byteLength) {
    slot.buffer?.destroy()
    slot.buffer = device.createBuffer({ size: data.byteLength, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST })
    slot.group = null
  }
  device.queue.writeBuffer(slot.buffer, 0, data)
  if (made.textured && !upload(slot)) return
  let group = slot.group
  if (group === null || slot.program !== shade.program) {
    const entries: GPUBindGroupEntry[] = [{ binding: 0, resource: { buffer: slot.buffer } }]
    if (made.textured && slot.texture !== null) {
      sampler ??= device.createSampler({ magFilter: "linear", minFilter: "linear" })
      entries.push({ binding: 1, resource: slot.texture.createView() }, { binding: 2, resource: sampler })
    }
    group = device.createBindGroup({ layout: made.pipeline.getBindGroupLayout(0), entries })
    slot.group = group
    slot.program = shade.program
  }
  const encoder = device.createCommandEncoder()
  const pass = encoder.beginRenderPass({
    colorAttachments: [
      {
        view: slot.context.getCurrentTexture().createView(),
        loadOp: "clear",
        storeOp: "store",
        clearValue: clearColor(),
      },
    ],
  })
  pass.setPipeline(made.pipeline)
  pass.setBindGroup(0, group)
  pass.draw(3)
  pass.end()
  device.queue.submit([encoder.finish()])
}

function tint(hex: string): [number, number, number, number] {
  const found = tints.get(hex)
  if (found !== undefined) return found
  const raw = hex.replace("#", "")
  const part = (at: number, fallback: number) => (raw.length >= at + 2 ? parseInt(raw.slice(at, at + 2), 16) : fallback)
  const made: [number, number, number, number] = [part(0, 0), part(2, 0), part(4, 0), part(6, 255)]
  tints.set(hex, made)
  return made
}

function upload(slot: Slot): boolean {
  if (device === null) return false
  const rows = slot.node.rows
  const palette = (slot.node.palette ?? []).map(tint)
  const height = rows.length
  const width = rows[0]?.length ?? 0
  if (width === 0 || height === 0 || palette.length === 0) return false
  const pixels = new Uint8Array(width * height * 4)
  for (let y = 0; y < height; y++) {
    const row = rows[y] as number[]
    for (let x = 0; x < width; x++) {
      const [r, g, b, a] = palette[row[x] as number] ?? [0, 0, 0, 255]
      const i = (y * width + x) * 4
      pixels[i] = r
      pixels[i + 1] = g
      pixels[i + 2] = b
      pixels[i + 3] = a
    }
  }
  if (slot.texture === null || slot.texture.width !== width || slot.texture.height !== height) {
    slot.texture?.destroy()
    slot.texture = device.createTexture({
      size: [width, height],
      format: "rgba8unorm-srgb",
      usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST,
    })
    slot.group = null
  }
  device.queue.writeTexture({ texture: slot.texture }, pixels, { bytesPerRow: width * 4 }, [width, height])
  return true
}
