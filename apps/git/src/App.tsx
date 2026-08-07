import { useEffect, useState } from "react"
import { Skeleton, useLinks, useRoute, useScrolled } from "mrlyui"
import { load } from "./lib/data"
import type { Ctx } from "./lib/repo"
import { fromManifest } from "./lib/repo"
import { Shell } from "./components/Shell"
import { NotFound } from "./views/NotFound"
import { Route } from "./views/Route"

const NONE = fromManifest({ dirs: { "": {} }, meta: {} })

export function App() {
  const [ctx, set] = useState<Ctx | undefined>(undefined)
  const route = useRoute()
  useScrolled()
  useLinks()
  useEffect(() => {
    void load()
      .then(set)
      .catch(() => set(NONE))
  }, [])
  if (ctx === undefined) {
    return (
      <Shell ctx={NONE} route={route} title="" desc="">
        <Skeleton head lines={3} block />
      </Shell>
    )
  }
  const known = ctx.dirs.has(route) || ctx.files.has(route)
  return known ? <Route ctx={ctx} route={route} /> : <NotFound ctx={ctx} />
}
