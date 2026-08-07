import { mkdir, writeFile } from "node:fs/promises"
import { join, resolve } from "node:path"
import { BASE, CATALOG, ORDERS } from "../src/lib/site"
import { scan } from "./scan"

// PATHS

const APP = resolve(import.meta.dir, "..")

const OUT = resolve(APP, process.env["MRLY_OUT"] ?? "../../data/net/dist")

// SITEMAP

function sitemap(routes: string[]): string {
  const urls = routes.map(route =>
    route === ""
      ? `  <url><loc>${BASE}/</loc><priority>1.0</priority></url>`
      : `  <url><loc>${BASE}/${route}</loc></url>`,
  )
  return [
    '<?xml version="1.0" encoding="UTF-8"?>',
    '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">',
    ...urls,
    "</urlset>",
    "",
  ].join("\n")
}

// MAIN

const { site } = scan(APP)
const routes = ["", ...Object.keys(site.products), ...Object.keys(site.pages), CATALOG.route, ORDERS.route]
await mkdir(OUT, { recursive: true })
await writeFile(join(OUT, "sitemap.xml"), sitemap(routes))
await writeFile(join(OUT, "site.json"), JSON.stringify(site))
console.log(`net: ${routes.length} routes -> ${OUT}`)
