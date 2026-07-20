type GPUTextureFormat = string

type GPUPrimitiveTopology = "triangle-list" | "line-list"

type GPUVertexFormat = "float32" | "float32x3" | "float32x4"

type GPUBlendFactor = "one" | "one-minus-src-alpha"

interface GPU {
  requestAdapter(): Promise<GPUAdapter | null>
  getPreferredCanvasFormat(): GPUTextureFormat
}

interface GPUAdapter {
  requestDevice(): Promise<GPUDevice>
}

interface GPUQueue {
  writeBuffer(buffer: GPUBuffer, offset: number, data: ArrayBufferView): void
  writeTexture(
    destination: { texture: GPUTexture },
    data: ArrayBufferView,
    layout: { bytesPerRow: number },
    size: [number, number],
  ): void
  submit(commands: GPUCommandBuffer[]): void
}

type GPUVertexAttribute = { shaderLocation: number; offset: number; format: GPUVertexFormat }

type GPUVertexBufferLayout = { arrayStride: number; attributes: GPUVertexAttribute[] }

type GPUBlendComponent = { srcFactor: GPUBlendFactor; dstFactor: GPUBlendFactor }

type GPUBlendState = { color: GPUBlendComponent; alpha: GPUBlendComponent }

type GPUDepthStencilState = { format: GPUTextureFormat; depthWriteEnabled: boolean; depthCompare: "less" }

type GPUBindGroupLayoutEntry = { binding: number; visibility: number; buffer: object }

interface GPUDevice {
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

interface GPUShaderModule {
  readonly label: string
}

interface GPURenderPipeline {
  getBindGroupLayout(index: number): GPUBindGroupLayout
}

interface GPUPipelineLayout {
  readonly label: string
}

interface GPUBindGroupLayout {
  readonly label: string
}

interface GPUBindGroup {
  readonly label: string
}

interface GPUBuffer {
  readonly size: number
  destroy(): void
}

interface GPUTexture {
  readonly width: number
  readonly height: number
  createView(): GPUTextureView
  destroy(): void
}

interface GPUTextureView {
  readonly label: string
}

interface GPUSampler {
  readonly label: string
}

type GPUBindGroupEntry = {
  binding: number
  resource: { buffer: GPUBuffer } | GPUSampler | GPUTextureView
}

interface GPUCommandEncoder {
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

interface GPURenderPassEncoder {
  setPipeline(pipeline: GPURenderPipeline): void
  setBindGroup(index: number, group: GPUBindGroup): void
  setVertexBuffer(slot: number, buffer: GPUBuffer): void
  draw(count: number): void
  end(): void
}

interface GPUCommandBuffer {
  readonly label: string
}

interface GPUCanvasContext {
  configure(configuration: { device: GPUDevice; format: GPUTextureFormat; alphaMode?: string }): void
  getCurrentTexture(): GPUTexture
}

interface Navigator {
  readonly gpu?: GPU
}

declare var GPUBufferUsage: { UNIFORM: number; COPY_DST: number; VERTEX: number }
declare var GPUTextureUsage: { TEXTURE_BINDING: number; COPY_DST: number; RENDER_ATTACHMENT: number }
declare var GPUShaderStage: { VERTEX: number; FRAGMENT: number }
