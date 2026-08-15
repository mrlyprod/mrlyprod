import { useEffect, useState } from "react"
import { Crumbs, Title, useLinks, useRoute, useScrolled } from "mrlyui"
import { Landing, Products } from "./components/Landing"
import { Menu } from "./components/Menu"
import { Shell } from "./components/Shell"
import { load, NONE } from "./lib/data"
import type { Site } from "./lib/data"
import { CATALOG, FILM, ROOT } from "./lib/site"
import { Focus } from "./views/Focus"
import { NotFound } from "./views/NotFound"
import { Page } from "./views/Page"

// VIEWS

function Home({ site }: { site: Site }) {
  return (
    <Shell site={site} route="" title={site.home[0]} desc={site.home[1]} wide>
      <Landing site={site} />
    </Shell>
  )
}

function Catalog({ site }: { site: Site }) {
  return (
    <Shell site={site} route={CATALOG.route} title={CATALOG.title} desc={CATALOG.desc}>
      <Crumbs root={ROOT} route={CATALOG.route} />
      <Title>Menu</Title>
      <Menu site={site} route={CATALOG.route} />
      <Products site={site} />
    </Shell>
  )
}

// APP

export function App() {
  const [site, set] = useState<Site | undefined>(undefined)
  const route = useRoute()
  useScrolled()
  useLinks()
  useEffect(() => {
    void load()
      .then(set)
      .catch(() => set(NONE))
  }, [])
  if (site === undefined) {
    return (
      <Shell site={NONE} route={route} title={ROOT} desc="">
        {null}
      </Shell>
    )
  }
  if (route.startsWith(FILM) && route.length > FILM.length) {
    return <Focus site={site} name={route.slice(FILM.length)} />
  }
  const product = site.products[route]
  if (product !== undefined) return <Page site={site} doc={{ route, title: product[0], desc: product[1] }} product />
  const page = site.pages[route]
  if (page !== undefined) return <Page site={site} doc={{ route, title: page[0], desc: page[1] }} />
  switch (route) {
    case "":
    case "home":
      return <Home site={site} />
    case CATALOG.route:
      return <Catalog site={site} />
    default:
      return <NotFound site={site} />
  }
}
