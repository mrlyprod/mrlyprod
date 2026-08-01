import type { Link } from "mrlydom"
import type { Site } from "./data"

// SITE

export const BASE = "https://mrly.net"

export const ROOT = "mrly.net"

export const HELP = "/contact"

export const BUILD = "build"

export const LEAD = "Estimated lead time: 3 months."

export type Step = readonly [string, string]

export const FUNNEL: Step[] = [
  ["try", "https://web.mrly.net"],
  ["read", "https://git.mrly.net"],
  ["own", "https://git.mrly.net/install"],
]

export const PLACES: Link[] = [
  ["home", "/", "home"],
  ["about", "/about", "info"],
  ["contact", "/contact", "mail"],
  ["privacy", "/privacy", "shield"],
  ["terms", "/terms", "gavel"],
]

export const SOCIALS: Link[] = [
  ["instagram", "https://instagram.com/mrlyprod", "instagram"],
  ["reddit", "https://reddit.com/r/mrlyprod", "reddit"],
  ["twitter", "https://twitter.com/mrlyprod", "x"],
  ["github", "https://github.com/mrlyprod", "github"],
  ["discord", "https://discord.gg/YEKjjvwhcK", "discord"],
  ["tiktok", "https://tiktok.com/@mrlyprod", "tiktok"],
  ["youtube", "https://youtube.com/@mrlyprod", "youtube"],
  ["donate", "https://donate.stripe.com/dRm3cu3XLfHj19e6WW5kk00", "volunteer_activism"],
  ["help", "mailto:help@mrlyprod.com", "alternate_email"],
]

// KEYS

export const PRESET = "build"

export const BASKET = "mrly-orders"

// DOCS

export type Doc = { route: string; title: string; desc: string }

export const DESIGNED = ["math", "bricks", "sheets"]

export const PLAIN = ["about", "contact", "privacy", "terms"]

export const DESIGNER: Doc = { route: BUILD, title: "Build", desc: "Shape a brick or a sheet in the browser." }

export const CATALOG: Doc = { route: "menu", title: "Menu", desc: `Every page on ${ROOT}.` }

export const ORDERS: Doc = { route: "cart", title: "Cart", desc: "Pre-orders waiting to be placed." }

// MENU

export type Nav = { cls: string; name: string; href: string; current: boolean }

function lead(name: string, route: string): Nav {
  return { cls: "lead", name, href: `/${name}`, current: route === name }
}

export function menuLinks(route: string, site: Site): Nav[] {
  const out: Nav[] = [{ cls: "lead", name: "home", href: "/", current: route === "" }]
  for (const name of Object.keys(site.products)) out.push(lead(name, route))
  out.push(lead(BUILD, route), lead("research", route))
  out.push({ cls: "lead", name: "blog", href: "/blog", current: route === "blog" || route.startsWith("blog/") })
  for (const [name, href] of FUNNEL.slice(0, 2)) out.push({ cls: "out", name, href, current: false })
  for (const name of PLAIN) out.push({ cls: "fine", name, href: `/${name}`, current: route === name })
  return out
}

// PRICES

export type Tier = { id: string; label: string; max: number; price: number; url: string }

export const PRICES: Record<string, Tier[]> = {
  math: [
    { id: "s", label: "Small", max: 9, price: 240, url: "" },
    { id: "m", label: "Medium", max: 32, price: 640, url: "" },
  ],
  bricks: [
    { id: "s", label: "Small", max: 9, price: 190, url: "" },
    { id: "m", label: "Medium", max: 32, price: 490, url: "" },
  ],
  sheets: [
    { id: "s", label: "Small", max: 256, price: 140, url: "" },
    { id: "m", label: "Medium", max: 1024, price: 290, url: "" },
    { id: "l", label: "Large", max: 2304, price: 540, url: "" },
  ],
}
