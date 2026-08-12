import { createReadStream, existsSync, statSync } from "node:fs"
import { readFile } from "node:fs/promises"
import { join } from "node:path"
import type { Plugin } from "vite"
import type { Scan } from "./scan"
import { scan } from "./scan"

// TYPES

const BINARY: Record<string, string> = {
  gif: "image/gif",
  ico: "image/x-icon",
  jpeg: "image/jpeg",
  jpg: "image/jpeg",
  pdf: "application/pdf",
  png: "image/png",
  svg: "image/svg+xml",
  ttf: "font/ttf",
  wasm: "application/wasm",
  webp: "image/webp",
  woff: "font/woff",
  woff2: "font/woff2",
}

function mime(path: string): string {
  const name = path.split("/").pop() ?? ""
  const ext = name.includes(".") ? (name.split(".").pop() ?? "").toLowerCase() : ""
  return BINARY[ext] ?? "text/plain; charset=utf-8"
}

// CACHE

const TTL = 2000

let held: Scan | undefined
let stamp = 0

function fresh(): Scan {
  const now = Date.now()
  if (held === undefined || now - stamp > TTL) {
    held = scan()
    stamp = now
  }
  return held
}

// PLUGIN

export function repo(root: string): Plugin {
  const index = join(root, "index.html")
  const statics = join(root, "public")
  return {
    name: "git-repo",
    enforce: "pre",
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const url = (req.url ?? "/").split("?")[0] ?? "/"
        if (url === "/manifest.json") {
          res.setHeader("content-type", "application/json")
          res.setHeader("cache-control", "no-cache")
          res.end(JSON.stringify(fresh().manifest))
          return
        }
        if (url.startsWith("/raw/")) {
          const site = fresh()
          const target = decodeURIComponent(url.slice(5))
          const file = join(site.base, target)
          if (!site.ctx.tracked.has(target) || !existsSync(file)) {
            res.statusCode = 404
            res.end("not found")
            return
          }
          res.setHeader("content-type", mime(target))
          res.setHeader("cache-control", "no-cache")
          createReadStream(file).pipe(res)
          return
        }
        const html = (req.headers.accept ?? "").includes("text/html")
        if (req.method !== "GET" || !html || served(statics, url)) {
          next()
          return
        }
        void readFile(index, "utf8")
          .then(body => server.transformIndexHtml(url, body))
          .then(body => {
            res.setHeader("content-type", "text/html")
            res.setHeader("cache-control", "no-cache")
            res.end(body)
          })
          .catch(() => next())
      })
    },
  }
}

function served(statics: string, url: string): boolean {
  const file = join(statics, decodeURIComponent(url))
  return existsSync(file) && statSync(file).isFile()
}
