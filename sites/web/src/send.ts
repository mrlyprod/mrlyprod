import { createContext, useContext } from "react"
import type { Manifest, Registry, Send } from "./types"

export const Wire = createContext<Send>(() => {})

export function useSend(): Send {
  return useContext(Wire)
}

const EMPTY: Registry = { version: "", apps: [], verbs: [], nav: [] }

export const Kernel = createContext<Registry>(EMPTY)

export function useKernel(): Registry {
  return useContext(Kernel)
}

export function useApps(): Manifest[] {
  return useContext(Kernel).apps
}

export function useApp(route: string): Manifest | undefined {
  return useContext(Kernel).apps.find(app => app.route === route)
}
