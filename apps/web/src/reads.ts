let store: (path: string) => unknown = () => null

export function install(fn: (path: string) => unknown): void {
  store = fn
}

export function reading(path: string): unknown {
  return store(path)
}
