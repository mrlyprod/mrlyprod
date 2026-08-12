import type { Call } from "./types"

const DB = "mrly"
const STORE = "journals"
const LIMIT = 500

export const RUNS: Record<string, number> = {
  "crush.step": 1024,
  "escape.step": 1024,
  "julia.step": 1024,
  "life.step": 1024,
  "mandelbrot.step": 1024,
  "matrix.step": 1024,
  "sleep.step": 1024,
  "snake.step": 1024,
  "solids.step": 1024,
  "tennis.step": 1024,
}

export type Journal = {
  iden: string
  version: string
  snapshot?: unknown
  calls: Call[]
}

// STOPS

function span(made: Call): number {
  if (RUNS[made.verb] === undefined) return 1
  const held = (made.args as { n?: unknown }).n
  return typeof held === "number" ? held : 1
}

export function stops(journal: Journal): number {
  let total = 0
  for (const made of journal.calls) total += span(made)
  return total
}

export function upto(journal: Journal, at: number): Call[] {
  const taken: Call[] = []
  let left = at
  for (const made of journal.calls) {
    if (left <= 0) break
    const size = span(made)
    if (size <= left) {
      taken.push(made)
      left -= size
      continue
    }
    taken.push({ ...made, args: { ...made.args, n: left } })
    left = 0
  }
  return taken
}

// STORE

function settle<T>(request: IDBRequest<T>): Promise<T> {
  return new Promise((resolve, reject) => {
    request.onsuccess = () => resolve(request.result)
    request.onerror = () => reject(request.error)
  })
}

async function database(): Promise<IDBDatabase> {
  const request = indexedDB.open(DB, 1)
  request.onupgradeneeded = () => {
    request.result.createObjectStore(STORE, { keyPath: "iden" })
  }
  return settle(request)
}

export class Store {
  private db: IDBDatabase
  private run: boolean
  journal: Journal
  discarded: boolean

  private constructor(db: IDBDatabase, journal: Journal, discarded: boolean) {
    this.db = db
    this.run = false
    this.journal = journal
    this.discarded = discarded
  }

  static async open(iden: string, version: string): Promise<Store> {
    const db = await database()
    const found = (await settle(db.transaction(STORE).objectStore(STORE).get(iden))) as Journal | undefined
    if (found !== undefined && found.version === version) return new Store(db, found, false)
    const fresh: Journal = { iden, version, calls: [] }
    return new Store(db, fresh, found !== undefined)
  }

  static async wipe(iden: string): Promise<void> {
    const db = await database()
    await settle(db.transaction(STORE, "readwrite").objectStore(STORE).delete(iden))
    db.close()
  }

  static async put(journal: Journal): Promise<void> {
    const db = await database()
    await settle(db.transaction(STORE, "readwrite").objectStore(STORE).put(journal))
    db.close()
  }

  record(call: Call, beat = false): void {
    const cap = beat ? RUNS[call.verb] : undefined
    const last = this.journal.calls[this.journal.calls.length - 1]
    if (cap !== undefined && this.run && last !== undefined && last.verb === call.verb) {
      const held = Number((last.args as { n?: unknown }).n ?? 1)
      const add = Number((call.args as { n?: unknown }).n ?? 1)
      if (held + add <= cap) {
        last.args = { n: held + add }
        last.now = call.now
        this.persist()
        return
      }
    }
    this.journal.calls.push(call)
    this.run = cap !== undefined
    this.persist()
  }

  fork(calls: Call[]): void {
    this.journal.calls = calls
    this.run = false
    this.persist()
  }

  full(): boolean {
    return this.journal.calls.length >= LIMIT
  }

  compact(snapshot: unknown): void {
    this.journal.snapshot = snapshot
    this.journal.calls = []
    this.run = false
    this.persist()
  }

  private persist(): void {
    this.db.transaction(STORE, "readwrite").objectStore(STORE).put(this.journal)
  }
}
