import { useCallback, useEffect, useRef, useState } from "react"
import { go, useLinks, useRoute, useScrolled } from "mrlydom"
import { load } from "./lib/data"
import type { Ctx } from "./lib/repo"
import { fromManifest, routes } from "./lib/repo"
import { useChrome } from "./lib/theme"
import { Shell } from "./components/Shell"
import { Skeleton } from "./components/Skeleton"
import { NotFound } from "./views/NotFound"
import { Route } from "./views/Route"

const NONE = fromManifest({ dirs: { "": {} }, meta: {} })

export function App() {
  const [ctx, set] = useState<Ctx | undefined>(undefined)
  const held = useRef<Ctx | undefined>(undefined)
  held.current = ctx
  const route = useRoute()
  const stray = useCallback(() => {
    const site = held.current
    if (site === undefined) {
      go("/")
      return
    }
    const all = routes(site)
      .map(target => (target === "" ? "/" : `/${target}`))
      .filter(href => href !== location.pathname)
    go(all[Math.floor(Math.random() * all.length)] ?? "/")
  }, [])
  useChrome(stray)
  useScrolled()
  useLinks()
  useEffect(() => {
    void load()
      .then(set)
      .catch(() => set(NONE))
  }, [])
  if (ctx === undefined) {
    return (
      <Shell ctx={NONE} route={route} current=" " title="" desc="">
        <Skeleton />
      </Shell>
    )
  }
  const known = ctx.dirs.has(route) || ctx.files.has(route)
  return known ? <Route ctx={ctx} route={route} /> : <NotFound ctx={ctx} />
}
