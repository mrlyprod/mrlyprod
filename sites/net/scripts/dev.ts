import { createReadStream, existsSync } from "node:fs"
import { join } from "node:path"
import type { Plugin } from "vite"
import { scan } from "./scan"

// PLUGIN

const RAW = "/raw/sites/net/"

export function content(root: string): Plugin {
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
        if (url.startsWith(RAW)) {
          const target = decodeURIComponent(url.slice(RAW.length))
          const known =
            (target.startsWith("pages/") || target.startsWith("blog/")) &&
            target.endsWith(".md") &&
            !target.includes("..")
          const file = join(root, target)
          if (!known || !existsSync(file)) {
            res.statusCode = 404
            res.end("not found")
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
