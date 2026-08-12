import type { Ctx, Manifest } from "./repo"
import { fromManifest } from "./repo"
import { CDN, RAW } from "./site"

// MANIFEST

export async function load(): Promise<Ctx> {
  const res = await fetch(`${CDN}manifest.json`)
  if (!res.ok) throw new Error(`git (manifest ${String(res.status)})`)
  return fromManifest((await res.json()) as Manifest)
}

// RAW

const held = new Map<string, Promise<string>>()

export function raw(path: string): Promise<string> {
  let body = held.get(path)
  if (body === undefined) {
    body = fetch(`${RAW}${path}`).then(res => {
      if (!res.ok) throw new Error(`git (${path}: ${String(res.status)})`)
      return res.text()
    })
    held.set(path, body)
  }
  return body
}
