import { useEffect, useState } from "react"

// STATE

const MOVE = "mrly-move"

export function here(): string {
  let path = location.pathname
  try {
    path = decodeURIComponent(path)
  } catch {}
  return path.replace(/^\/+/, "").replace(/\/+$/, "")
}

export function go(href: string): void {
  history.pushState(null, "", href)
  dispatchEvent(new Event(MOVE))
  document.querySelector<HTMLElement>(".main")?.scrollTo(0, 0)
  scrollTo(0, 0)
}

export function useRoute(): string {
  const [route, set] = useState(here)
  useEffect(() => {
    const sync = (): void => set(here())
    addEventListener("popstate", sync)
    addEventListener(MOVE, sync)
    return () => {
      removeEventListener("popstate", sync)
      removeEventListener(MOVE, sync)
    }
  }, [])
  return route
}

// LINKS

const RAW = "/raw/"

function inside(anchor: HTMLAnchorElement): boolean {
  if (anchor.target !== "" || anchor.hasAttribute("download")) return false
  const href = anchor.getAttribute("href")
  if (href === null || href === "" || href.startsWith("#")) return false
  const url = new URL(anchor.href)
  if (url.origin !== location.origin) return false
  if (url.pathname.startsWith(RAW)) return false
  return url.hash === "" || url.pathname !== location.pathname
}

export function useLinks(): void {
  useEffect(() => {
    const click = (event: MouseEvent): void => {
      if (event.defaultPrevented || event.button !== 0) return
      if (event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return
      const target = event.target instanceof Element ? event.target.closest("a") : null
      if (!(target instanceof HTMLAnchorElement) || !inside(target)) return
      event.preventDefault()
      const pane = target.closest<HTMLElement>("[popover]")
      if (pane !== null && typeof pane.hidePopover === "function" && pane.matches(":popover-open")) pane.hidePopover()
      const url = new URL(target.href)
      go(url.pathname + url.hash)
    }
    document.addEventListener("click", click)
    return () => document.removeEventListener("click", click)
  }, [])
}

// HEAD

type Head = {
  route: string
  title: string
  desc: string
  root: string
  base: string
}

function meta(name: string, value: string): void {
  document.querySelector(`meta[name="${name}"]`)?.setAttribute("content", value)
}

export function useHead({ route, title, desc, root, base }: Head): void {
  useEffect(() => {
    if (title === "" && route !== "") return
    document.title = route === "" ? root : `${title} \u2014 ${root}`
    meta("description", desc)
    const canon = route === "" ? `${base}/` : `${base}/${route}`
    document.querySelector('link[rel="canonical"]')?.setAttribute("href", canon)
  }, [route, title, desc, root, base])
}
