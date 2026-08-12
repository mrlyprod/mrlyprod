import type { BrandName, SymbolName } from "mrlyui"
import type { Site } from "./data"

// SITE

export type Link = readonly [string, string, BrandName | SymbolName]

export const BASE = "https://mrly.net"

export const ROOT = "mrly.net"

export const HELP = "/contact"

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

// DOCS

export type Doc = { route: string; title: string; desc: string }

export const DESIGNED = ["math", "bricks", "sheets"]

export const PLAIN = ["about", "contact", "privacy", "terms"]

export const CATALOG: Doc = { route: "menu", title: "Menu", desc: `Every page on ${ROOT}.` }

// MENU

export type Nav = { cls: string; name: string; href: string; current: boolean }

function lead(name: string, route: string): Nav {
  return { cls: "lead", name, href: `/${name}`, current: route === name }
}

export function menuLinks(route: string, site: Site): Nav[] {
  const out: Nav[] = [{ cls: "lead", name: "home", href: "/", current: route === "" }]
  for (const name of Object.keys(site.products)) out.push(lead(name, route))
  out.push(lead("research", route))
  out.push({ cls: "lead", name: "blog", href: "/blog", current: route === "blog" || route.startsWith("blog/") })
  for (const [name, href] of FUNNEL.slice(0, 2)) out.push({ cls: "out", name, href, current: false })
  for (const name of PLAIN) out.push({ cls: "fine", name, href: `/${name}`, current: route === name })
  return out
}
