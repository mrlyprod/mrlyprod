import { spawnSync } from "node:child_process"
import { createReadStream, existsSync, mkdirSync, readFileSync, statSync } from "node:fs"
import { basename, extname, join, resolve } from "node:path"
import type { ServerResponse } from "node:http"
import type { Plugin } from "vite"
import { scan } from "./scan"

// PATHS

const RAW = "/raw/sites/net/"

const VIDEOS = "/videos/"

const TYPES: Record<string, string> = {
  ".jpg": "image/jpeg",
  ".json": "application/json",
  ".mp4": "video/mp4",
}

// FIXTURES

function seedsOf(root: string): Map<string, string> {
  const out = new Map<string, string>()
  const base = join(root, "fixtures")
  const index = join(base, "videos", "index.json")
  if (!existsSync(index)) return out
  const reel = JSON.parse(readFileSync(index, "utf8")) as { chunks?: string[] }
  for (const path of reel.chunks ?? []) {
    const file = join(base, path)
    if (!existsSync(file)) continue
    const films = JSON.parse(readFileSync(file, "utf8")) as Array<{ name?: string; seed?: string }>
    for (const film of films) {
      if (film.name !== undefined && film.seed !== undefined) out.set(film.name, film.seed)
    }
  }
  return out
}

function grain(seed: string): string {
  try {
    return String(BigInt(seed) % 65536n)
  } catch {
    return "7"
  }
}

function press(seed: string, mp4: string, jpg: string): boolean {
  const life = [
    "life=size=32x32",
    "mold=16",
    "r=8",
    "ratio=0.2",
    "death_color=#0e0e0e",
    "life_color=#f2f2f2",
    "mold_color=#3a3a3a",
    `seed=${grain(seed)}`,
  ].join(":")
  const reel = spawnSync("ffmpeg", [
    ...["-v", "error", "-y", "-f", "lavfi", "-i", life, "-t", "3"],
    ...["-vf", "scale=160:160:flags=neighbor", "-c:v", "libx264", "-preset", "veryfast"],
    ...["-crf", "34", "-pix_fmt", "yuv420p", "-movflags", "+faststart", mp4],
  ])
  if (reel.status !== 0) return false
  const still = spawnSync("ffmpeg", ["-v", "error", "-y", "-i", mp4, "-frames:v", "1", "-q:v", "6", jpg])
  return still.status === 0
}

function madeOf(store: string, seeds: Map<string, string>, rest: string): string | undefined {
  const kind = rest.slice(0, 1)
  const name = basename(rest, extname(rest))
  const seed = seeds.get(name)
  if (seed === undefined || (kind !== "v" && kind !== "p")) return undefined
  const mp4 = join(store, `${name}.mp4`)
  const jpg = join(store, `${name}.jpg`)
  if (!existsSync(mp4) || !existsSync(jpg)) {
    mkdirSync(store, { recursive: true })
    if (!press(seed, mp4, jpg)) return undefined
  }
  return kind === "v" ? mp4 : jpg
}

// SERVE

function send(res: ServerResponse, file: string): void {
  res.setHeader("content-type", TYPES[extname(file)] ?? "application/octet-stream")
  res.setHeader("content-length", String(statSync(file).size))
  res.setHeader("cache-control", "no-cache")
  createReadStream(file).pipe(res)
}

function miss(res: ServerResponse): void {
  res.statusCode = 404
  res.end("not found")
}

// PLUGIN

export function content(root: string): Plugin {
  const store = resolve(root, "../../data/net/fixtures")
  return {
    name: "net-content",
    enforce: "pre",
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const url = (req.url ?? "/").split("?")[0] ?? "/"
        if (url === "/site.json") {
          res.setHeader("content-type", "application/json")
          res.setHeader("cache-control", "no-cache")
          res.end(JSON.stringify(scan(root).site))
          return
        }
        if (url.startsWith(VIDEOS)) {
          const rest = decodeURIComponent(url.slice(VIDEOS.length))
          if (rest === "" || rest.includes("..")) {
            miss(res)
            return
          }
          if (rest.endsWith(".json")) {
            const file = join(root, "fixtures", "videos", rest)
            if (existsSync(file)) send(res, file)
            else miss(res)
            return
          }
          const made = madeOf(store, seedsOf(root), rest)
          if (made === undefined) miss(res)
          else send(res, made)
          return
        }
        if (url.startsWith(RAW)) {
          const target = decodeURIComponent(url.slice(RAW.length))
          const known =
            (target.startsWith("pages/") || target.startsWith("blog/")) &&
            target.endsWith(".md") &&
            !target.includes("..")
          const file = join(root, target)
          if (!known || !existsSync(file)) {
            miss(res)
            return
          }
          res.setHeader("content-type", "text/plain; charset=utf-8")
          res.setHeader("cache-control", "no-cache")
          createReadStream(file).pipe(res)
          return
        }
        next()
      })
    },
  }
}
