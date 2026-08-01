import { lazy, Suspense, useEffect, useState } from "react"
import { Crumbs, useLinks, useRoute, useScrolled } from "mrlydom"
import { Cart } from "./components/Cart"
import { Landing } from "./components/Landing"
import { Menu } from "./components/Menu"
import { Shell } from "./components/Shell"
import { load, NONE } from "./lib/data"
import type { Site } from "./lib/data"
import { CATALOG, DESIGNER, ORDERS, ROOT } from "./lib/site"
import { NotFound } from "./views/NotFound"
import { Page } from "./views/Page"

const Build = lazy(() => import("./components/Build").then(held => ({ default: held.Build })))

// VIEWS

function Home({ site }: { site: Site }) {
  return (
    <Shell site={site} route="" title={site.home[0]} desc={site.home[1]}>
      <Landing site={site} />
    </Shell>
  )
}

function Designer({ site }: { site: Site }) {
  return (
    <Shell site={site} route={DESIGNER.route} title={DESIGNER.title} desc={DESIGNER.desc}>
      <Crumbs root={ROOT} route={DESIGNER.route} />
      <div className="product">
        <h1>Build</h1>
        <p>Set the numbers, turn the shape. Ordering comes later.</p>
      </div>
      <Suspense fallback={null}>
        <Build />
      </Suspense>
    </Shell>
  )
}

function Basket({ site }: { site: Site }) {
  return (
    <Shell site={site} route={ORDERS.route} title={ORDERS.title} desc={ORDERS.desc}>
      <Crumbs root={ROOT} route={ORDERS.route} />
      <h1>Cart</h1>
      <Cart />
    </Shell>
  )
}

function Catalog({ site }: { site: Site }) {
  return (
    <Shell site={site} route={CATALOG.route} title={CATALOG.title} desc={CATALOG.desc}>
      <Crumbs root={ROOT} route={CATALOG.route} />
      <h1>Menu</h1>
      <Menu site={site} route={CATALOG.route} sheet />
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
      <Shell site={NONE} route={route} title={ROOT} desc="" cart={false}>
        {null}
      </Shell>
    )
  }
  const product = site.products[route]
  if (product !== undefined) return <Page site={site} doc={{ route, title: product[0], desc: product[1] }} product />
  const page = site.pages[route]
  if (page !== undefined) return <Page site={site} doc={{ route, title: page[0], desc: page[1] }} />
  switch (route) {
    case "":
    case "home":
      return <Home site={site} />
    case DESIGNER.route:
      return <Designer site={site} />
    case ORDERS.route:
      return <Basket site={site} />
    case CATALOG.route:
      return <Catalog site={site} />
    default:
      return <NotFound site={site} />
  }
}
