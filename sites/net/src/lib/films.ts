import { useCallback, useEffect, useRef, useState } from "react"
import { CDN } from "./data"

// SHAPE

export type Film = {
  name: string
  seed: string
  video: string
  poster: string
  duration: number
  frames: number
}

export type Reel = { count: number; chunk: number; chunks: string[] }

export const INDEX = "videos/index.json"

export function asset(path: string): string {
  return `${CDN}${path.replace(/^\/+/, "")}`
}

// PARSE

function text(value: unknown): string {
  return typeof value === "string" ? value : ""
}

function tally(value: unknown): number {
  return typeof value === "number" && Number.isFinite(value) && value > 0 ? value : 0
}

function filmOf(value: unknown): Film | undefined {
  if (typeof value !== "object" || value === null) return undefined
  const raw = value as Record<string, unknown>
  const name = text(raw["name"])
  const video = text(raw["video"])
  const poster = text(raw["poster"])
  if (name === "" || video === "" || poster === "") return undefined
  const seed = text(raw["seed"])
  return { name, seed, video, poster, duration: tally(raw["duration"]), frames: tally(raw["frames"]) }
}

// READ

async function json(path: string): Promise<unknown> {
  const res = await fetch(asset(path))
  if (!res.ok) throw new Error(`films (${path}: ${String(res.status)})`)
  return (await res.json()) as unknown
}

export async function reel(): Promise<Reel> {
  const raw = await json(INDEX)
  const body = (typeof raw === "object" && raw !== null ? raw : {}) as Record<string, unknown>
  const list = body["chunks"]
  const chunks = Array.isArray(list) ? list.map(text).filter(one => one !== "") : []
  return { count: tally(body["count"]), chunk: tally(body["chunk"]) || 100, chunks }
}

export async function chunk(path: string): Promise<Film[]> {
  const raw = await json(path)
  if (!Array.isArray(raw)) return []
  const out: Film[] = []
  for (const one of raw) {
    const film = filmOf(one)
    if (film !== undefined) out.push(film)
  }
  return out
}

// SHELF

type Shelf = { reel: Reel | undefined; films: Film[]; at: number; scroll: number; dead: boolean }

const shelf: Shelf = { reel: undefined, films: [], at: 0, scroll: 0, dead: false }

const named = new Map<string, Film>()

function file(films: Film[]): void {
  for (const film of films) named.set(film.name, film)
}

export function held(name: string): Film | undefined {
  return named.get(name)
}

export function offset(): number {
  return shelf.scroll
}

export function remember(top: number): void {
  shelf.scroll = top
}

export async function seek(name: string): Promise<Film | undefined> {
  const got = named.get(name)
  if (got !== undefined) return got
  const list = shelf.reel ?? (shelf.reel = await reel())
  for (const path of list.chunks) {
    file(await chunk(path))
    const hit = named.get(name)
    if (hit !== undefined) return hit
  }
  return undefined
}

// REEL

export type Roll = { films: Film[]; count: number; done: boolean; failed: boolean; pull: () => void }

export function useReel(): Roll {
  const [, tick] = useState(0)
  const [failed, setFailed] = useState(shelf.dead)
  const busy = useRef(false)
  const pull = useCallback(() => {
    if (busy.current || shelf.dead) return
    const list = shelf.reel
    if (list !== undefined && shelf.at >= list.chunks.length) return
    busy.current = true
    void (async () => {
      try {
        const found = shelf.reel ?? (shelf.reel = await reel())
        const path = found.chunks[shelf.at]
        if (path !== undefined) {
          const batch = await chunk(path)
          file(batch)
          shelf.films = [...shelf.films, ...batch]
          shelf.at += 1
        }
      } catch {
        shelf.dead = true
        setFailed(true)
      }
      busy.current = false
      tick(count => count + 1)
    })()
  }, [])
  useEffect(() => {
    if (shelf.reel === undefined) pull()
  }, [pull])
  const list = shelf.reel
  return {
    films: shelf.films,
    count: list?.count ?? shelf.films.length,
    done: list !== undefined && shelf.at >= list.chunks.length,
    failed,
    pull,
  }
}
