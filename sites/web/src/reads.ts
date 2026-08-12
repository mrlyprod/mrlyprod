let store: (path: string) => unknown = () => null

export function install(fn: (path: string) => unknown): void {
  store = fn
}

export function reading(path: string): unknown {
  return store(path)
}

export function floats(path: string): number[] | undefined {
  const packed = store(path) as { dtype: string; data: string } | null
  if (packed === null || packed.dtype !== "f32") return undefined
  const bytes = Uint8Array.from(atob(packed.data), c => c.charCodeAt(0))
  return Array.from(new Float32Array(bytes.buffer))
}
