import type { Link } from "mrlydom"

// SITE

export const BASE = "https://git.mrly.net"

export const ROOT = "mrly.net"

export const CDN = import.meta.env.DEV ? "/" : "https://cdn.mrly.net/"

export const RAW = `${CDN}raw/`

export const CLONE = "curl -fsSL https://cdn.mrly.net/mrlyprod.tar.gz | tar xz"

export const PLACES: Link[] = [
  ["home", "/", "home"],
  ["tree", "/tree", "account_tree"],
  ["about", "https://mrly.net/about", "info"],
  ["contact", "https://mrly.net/contact", "mail"],
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

// META

export function raw(path: string): string {
  return `${RAW}${path}`
}
