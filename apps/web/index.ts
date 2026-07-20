import type { HTMLBundle } from "bun"
import { readdirSync } from "node:fs"
import { resolve } from "node:path"
import index from "./index.html"

const here = import.meta.dir
const publicDir = resolve(`${here}/public`)
const cdnDir = resolve(`${here}/../../cdn`)
const NO_STORE = { "Cache-Control": "no-store" }

const routes: Record<string, HTMLBundle | (() => Response)> = {
  "/": index,
  "/mrlyjs_bg.wasm": () =>
    new Response(Bun.file(`${here}/../../pkgs/mrlyjs/pkg/mrlyjs_bg.wasm`), { headers: NO_STORE }),
}
for (const entry of readdirSync(publicDir, { withFileTypes: true })) {
  if (!entry.isFile() || entry.name === ".DS_Store") continue
  const serve = () => new Response(Bun.file(`${publicDir}/${entry.name}`), { headers: NO_STORE })
  routes[`/${entry.name}`] = serve
  if (entry.name.endsWith(".html")) routes[`/${entry.name.slice(0, -5)}`] = serve
}
routes["/:app"] = index

const server = Bun.serve({
  port: Number(process.env.PORT) || 3000,
  development: true,
  routes,
  async fetch(req) {
    const pathname = new URL(req.url).pathname
    if (pathname.startsWith("/cdn/")) {
      const cdnPath = resolve(cdnDir, `.${pathname.slice(4)}`)
      if (!cdnPath.startsWith(cdnDir)) return new Response("Not Found", { status: 404, headers: NO_STORE })
      const asset = Bun.file(cdnPath)
      if (await asset.exists()) return new Response(asset, { headers: NO_STORE })
      return new Response("Not Found", { status: 404, headers: NO_STORE })
    }
    const path = resolve(publicDir, `.${pathname}`)
    if (!path.startsWith(publicDir)) return new Response("Not Found", { status: 404, headers: NO_STORE })
    const file = Bun.file(path)
    if (await file.exists()) return new Response(file, { headers: NO_STORE })
    const page = Bun.file(`${path}.html`)
    if (await page.exists()) return new Response(page, { headers: NO_STORE })
    return new Response(Bun.file(`${publicDir}/404.html`), { status: 404, headers: NO_STORE })
  },
})

console.log(`mrly at ${server.url}`)
