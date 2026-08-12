import { mkdir, writeFile } from "node:fs/promises"
import { join, resolve as resolvePath } from "node:path"
import { routeDate, routes } from "../src/lib/repo"
import { BASE } from "../src/lib/site"
import { scan } from "./scan"

// PATHS

const OUT = resolvePath(import.meta.dir, "..", process.env["MRLY_OUT"] ?? "../../data/git/dist")

// SITEMAP

function sitemap(rows: [string, string | undefined][]): string {
  const urls = rows.map(([route, date]) => {
    const lastmod = date === undefined ? "" : `<lastmod>${date}</lastmod>`
    return route === ""
      ? `  <url><loc>${BASE}/</loc>${lastmod}<priority>1.0</priority></url>`
      : `  <url><loc>${BASE}/${route}</loc>${lastmod}</url>`
  })
  return [
    '<?xml version="1.0" encoding="UTF-8"?>',
    '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">',
    ...urls,
    "</urlset>",
    "",
  ].join("\n")
}

// TREE

function tree(rows: [string, string | undefined][]): string {
  const cells = rows.map(([route, date]) => `"${route === "" ? "/" : `/${route}`}":"${date ?? ""}"`)
  return `{${cells.join(",")}}\n`
}

// MAIN

const { ctx, paths, manifest } = scan()
const rows = routes(ctx).map(route => [route, routeDate(ctx, route)] as [string, string | undefined])
await mkdir(OUT, { recursive: true })
await writeFile(join(OUT, "manifest.json"), JSON.stringify(manifest))
await writeFile(join(OUT, "sitemap.xml"), sitemap(rows))
await writeFile(join(OUT, "tree.json"), tree(rows))
console.log(`git: ${ctx.dirs.size} dirs, ${ctx.files.size} files, ${paths.length} tracked -> ${OUT}`)
