// SHAPE

export type Meta = readonly [title: string, desc: string]

export type Site = { home: Meta; products: Record<string, Meta>; pages: Record<string, Meta> }

export const NONE: Site = { home: ["mrly.net", ""], products: {}, pages: {} }

export function routesOf(site: Site): string[] {
  return [...Object.keys(site.products), ...Object.keys(site.pages)]
}

// PATHS

export function pathOf(route: string): string {
  if (route === "") return "pages/HOME.md"
  if (route === "blog") return "blog/README.md"
  if (route.startsWith("blog/")) return `${route}.md`
  return `pages/${route.toUpperCase()}.md`
}

// CDN

export const CDN = import.meta.env.DEV ? "/" : "https://cdn.mrly.net/"

export const RAW = `${CDN}raw/sites/net/`

export async function load(): Promise<Site> {
  const res = await fetch(`${CDN}site.json`)
  if (!res.ok) throw new Error(`net (site ${String(res.status)})`)
  return (await res.json()) as Site
}

const held = new Map<string, Promise<string>>()

export function raw(route: string): Promise<string> {
  let body = held.get(route)
  if (body === undefined) {
    const path = pathOf(route)
    body = fetch(`${RAW}${path}`).then(res => {
      if (!res.ok) throw new Error(`net (${path}: ${String(res.status)})`)
      return res.text()
    })
    held.set(route, body)
    body.catch(() => held.delete(route))
  }
  return body
}
