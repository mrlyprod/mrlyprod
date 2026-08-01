import { useEffect, useState } from "react"
import { Crumbs } from "mrlydom"
import type { Section } from "mrlydom"
import { Listing } from "../components/Listing"
import { More } from "../components/More"
import { Shell } from "../components/Shell"
import { Skeleton } from "../components/Skeleton"
import { Status } from "../components/Status"
import { raw } from "../lib/data"
import { lang } from "../lib/langs"
import type { Ctx } from "../lib/repo"
import { desc, lastSegment, title } from "../lib/repo"
import { RAW, ROOT } from "../lib/site"

// BODY

type Held = { html: string; toc: Section[]; lines: number }

const BLANK: Held = { html: "", toc: [], lines: 0 }

const cache = new Map<string, Held>()

async function build(ctx: Ctx, route: string): Promise<Held> {
  const file = ctx.files.get(route)
  const path = file === undefined ? ctx.dirs.get(route)?.readme : file.path
  if (path === undefined) return BLANK
  const body = await raw(path)
  if (file !== undefined && !file.doc) {
    const { source } = await import("../lib/code")
    return { html: await source(path, body), toc: [], lines: body.replace(/\n$/, "").split("\n").length }
  }
  const { render } = await import("../lib/md")
  const doc = await render(ctx, path, body)
  return { html: doc.html, toc: doc.toc, lines: 0 }
}

function useBody(ctx: Ctx, route: string): Held {
  const [, tick] = useState(0)
  useEffect(() => {
    if (cache.has(route)) return
    let live = true
    void build(ctx, route)
      .then(next => {
        cache.set(route, next)
        if (live) tick(n => n + 1)
      })
      .catch(() => {})
    return () => {
      live = false
    }
  }, [ctx, route])
  return cache.get(route) ?? BLANK
}

// ANCHOR

function useAnchor(route: string, ready: boolean): void {
  useEffect(() => {
    if (!ready || location.hash === "") return
    document.getElementById(decodeURIComponent(location.hash.slice(1)))?.scrollIntoView()
  }, [route, ready])
}

// ROUTE

export function Route({ ctx, route }: { ctx: Ctx; route: string }) {
  const file = ctx.files.get(route)
  const dir = ctx.dirs.get(route)
  const held = useBody(ctx, route)
  useAnchor(route, held.html !== "")
  const path = file === undefined ? dir?.readme : file.path
  const code = file !== undefined && !file.doc
  const name = code ? lastSegment(file.path) : ""
  const updated = ctx.updated.get(route)
  const status = code ? (
    <Status lang={lang(file.path)} lines={held.lines} />
  ) : (
    <Status updated={updated} />
  )
  return (
    <Shell
      ctx={ctx}
      route={route}
      title={title(ctx, route)}
      desc={desc(ctx, route)}
      toc={held.toc}
      status={status}
    >
      <Crumbs root={ROOT} route={route}>
        {path !== undefined && <More href={`${RAW}${path}`} />}
        {updated !== undefined && <span className="updated">{updated}</span>}
      </Crumbs>
      {name !== "" && <h1>{name}</h1>}
      {held.html !== "" && <div dangerouslySetInnerHTML={{ __html: held.html }} />}
      {held.html === "" && path !== undefined && <Skeleton />}
      {dir !== undefined && <Listing route={route} dir={dir} />}
    </Shell>
  )
}
