import { lazy } from "react"
import type { ComponentType, LazyExoticComponent } from "react"

// LOADERS

type Loader = () => Promise<{ default: ComponentType }>

const LOADERS: Record<string, Loader> = {
  proof: () => import("../exhibits/Proof"),
}

// LOOKUP

const cache = new Map<string, LazyExoticComponent<ComponentType>>()

export function exhibitOf(name: string): LazyExoticComponent<ComponentType> | null {
  const found = cache.get(name)
  if (found !== undefined) return found
  const load = LOADERS[name]
  if (load === undefined) return null
  const made = lazy(load)
  cache.set(name, made)
  return made
}
