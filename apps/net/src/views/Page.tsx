import { useEffect, useState } from "react"
import { Crumbs } from "mrlydom"
import type { Section } from "mrlydom"
import { Panel } from "../components/Panel"
import { Shell } from "../components/Shell"
import { raw, routesOf } from "../lib/data"
import type { Site } from "../lib/data"
import type { Doc } from "../lib/site"
import { DESIGNED, ROOT } from "../lib/site"

// BODY

type Held = { html: string; toc: Section[] }

const BLANK: Held = { html: "", toc: [] }

const cache = new Map<string, Held>()

function useBody(site: Site, doc: Doc): Held {
  const [, tick] = useState(0)
  useEffect(() => {
    if (cache.has(doc.route)) return
    let live = true
    void Promise.all([raw(doc.route), import("../lib/md")])
      .then(([body, { render }]) => render(doc.route, body, routesOf(site)))
      .then(next => {
        cache.set(doc.route, { html: next.html, toc: next.toc })
        if (live) tick(n => n + 1)
      })
      .catch(() => {})
    return () => {
      live = false
    }
  }, [site, doc.route])
  return cache.get(doc.route) ?? BLANK
}

// ANCHOR

function useAnchor(route: string, ready: boolean): void {
  useEffect(() => {
    if (!ready || location.hash === "") return
    document.getElementById(decodeURIComponent(location.hash.slice(1)))?.scrollIntoView()
  }, [route, ready])
}

// PAGE

export function Page({ site, doc, product = false }: { site: Site; doc: Doc; product?: boolean }) {
  const held = useBody(site, doc)
  useAnchor(doc.route, held.html !== "")
  return (
    <Shell site={site} route={doc.route} title={doc.title} desc={doc.desc} toc={held.toc}>
      <Crumbs root={ROOT} route={doc.route} />
      <div className={product ? "product" : undefined} dangerouslySetInnerHTML={{ __html: held.html }} />
      {product && <Panel preset={DESIGNED.includes(doc.route) ? doc.route : undefined} />}
    </Shell>
  )
}
