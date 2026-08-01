export type GPUTextureFormat = string

export type GPUPrimitiveTopology = "triangle-list" | "line-list"

export type GPUVertexFormat = "float32" | "float32x3" | "float32x4"

export type GPUBlendFactor = "one" | "one-minus-src-alpha"

export interface GPU {
  requestAdapter(): Promise<GPUAdapter | null>
  getPreferredCanvasFormat(): GPUTextureFormat
}

export interface GPUAdapter {
  requestDevice(): Promise<GPUDevice>
}

export interface GPUQueue {
  writeBuffer(buffer: GPUBuffer, offset: number, data: ArrayBufferView): void
  writeTexture(
    destination: { texture: GPUTexture },
    data: ArrayBufferView,
    layout: { bytesPerRow: number },
    size: [number, number],
  ): void
  submit(commands: GPUCommandBuffer[]): void
}

export type GPUVertexAttribute = { shaderLocation: number; offset: number; format: GPUVertexFormat }

export type GPUVertexBufferLayout = { arrayStride: number; attributes: GPUVertexAttribute[] }

export type GPUBlendComponent = { srcFactor: GPUBlendFactor; dstFactor: GPUBlendFactor }

export type GPUBlendState = { color: GPUBlendComponent; alpha: GPUBlendComponent }

export type GPUDepthStencilState = { format: GPUTextureFormat; depthWriteEnabled: boolean; depthCompare: "less" }

export type GPUBindGroupLayoutEntry = { binding: number; visibility: number; buffer: object }

export interface GPUDevice {
  readonly queue: GPUQueue
  createShaderModule(descriptor: { code: string }): GPUShaderModule
  createRenderPipeline(descriptor: {
    layout: "auto" | GPUPipelineLayout
    vertex: { module: GPUShaderModule; entryPoint: string; buffers?: GPUVertexBufferLayout[] }
    fragment: { module: GPUShaderModule; entryPoint: string; targets: { format: GPUTextureFormat; blend?: GPUBlendState }[] }
    primitive?: { topology: GPUPrimitiveTopology }
    depthStencil?: GPUDepthStencilState
  }): GPURenderPipeline
  createBuffer(descriptor: { size: number; usage: number }): GPUBuffer
  createTexture(descriptor: { size: [number, number]; format: GPUTextureFormat; usage: number }): GPUTexture
  createSampler(descriptor?: { magFilter?: string; minFilter?: string }): GPUSampler
  createBindGroup(descriptor: { layout: GPUBindGroupLayout; entries: GPUBindGroupEntry[] }): GPUBindGroup
  createBindGroupLayout(descriptor: { entries: GPUBindGroupLayoutEntry[] }): GPUBindGroupLayout
  createPipelineLayout(descriptor: { bindGroupLayouts: GPUBindGroupLayout[] }): GPUPipelineLayout
  createCommandEncoder(): GPUCommandEncoder
}

export interface GPUShaderModule {
  readonly label: string
}

export interface GPURenderPipeline {
  getBindGroupLayout(index: number): GPUBindGroupLayout
}

export interface GPUPipelineLayout {
  readonly label: string
}

export interface GPUBindGroupLayout {
  readonly label: string
}

export interface GPUBindGroup {
  readonly label: string
}

export interface GPUBuffer {
  readonly size: number
  destroy(): void
}

export interface GPUTexture {
  readonly width: number
  readonly height: number
  createView(): GPUTextureView
  destroy(): void
}

export interface GPUTextureView {
  readonly label: string
}

export interface GPUSampler {
  readonly label: string
}

export type GPUBindGroupEntry = {
  binding: number
  resource: { buffer: GPUBuffer } | GPUSampler | GPUTextureView
}

export interface GPUCommandEncoder {
  beginRenderPass(descriptor: {
    colorAttachments: {
      view: GPUTextureView
      loadOp: "clear" | "load"
      storeOp: "store" | "discard"
      clearValue?: { r: number; g: number; b: number; a: number }
    }[]
    depthStencilAttachment?: {
      view: GPUTextureView
      depthClearValue: number
      depthLoadOp: "clear" | "load"
      depthStoreOp: "store" | "discard"
    }
  }): GPURenderPassEncoder
  finish(): GPUCommandBuffer
}

export interface GPURenderPassEncoder {
  setPipeline(pipeline: GPURenderPipeline): void
  setBindGroup(index: number, group: GPUBindGroup): void
  setVertexBuffer(slot: number, buffer: GPUBuffer): void
  draw(count: number): void
  end(): void
}

export interface GPUCommandBuffer {
  readonly label: string
}

export interface GPUCanvasContext {
  configure(configuration: { device: GPUDevice; format: GPUTextureFormat; alphaMode?: string }): void
  getCurrentTexture(): GPUTexture
}

declare global {
  interface Navigator {
    readonly gpu?: GPU
  }
  var GPUBufferUsage: { UNIFORM: number; COPY_DST: number; VERTEX: number }
  var GPUTextureUsage: { TEXTURE_BINDING: number; COPY_DST: number; RENDER_ATTACHMENT: number }
  var GPUShaderStage: { VERTEX: number; FRAGMENT: number }
}
