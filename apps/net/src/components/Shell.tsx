import type { ReactNode } from "react"
import { Aside, Footer, Glyph, Letters, useHead } from "mrlydom"
import type { Section } from "mrlydom"
import { Cart } from "./Cart"
import { Menu } from "./Menu"
import type { Site } from "../lib/data"
import { BASE, PLACES, ROOT, SOCIALS } from "../lib/site"

type Props = {
  site: Site
  route: string
  title: string
  desc: string
  cart?: boolean
  toc?: Section[]
  children: ReactNode
}

// SHELL

export function Shell({ site, route, title, desc, cart = true, toc = [], children }: Props) {
  useHead({ route, title, desc, root: ROOT, base: BASE })
  return (
    <>
      <header>
        <button id="tree-btn" className="panel-btn" type="button" popoverTarget="explorer" aria-label="explorer">
          <Glyph name="plus" cls="glyph" />
          <Glyph name="minus" cls="glyph swap" />
        </button>
        <a className="lettermark" href="/" aria-label="home" title="home">
          <Letters cls="headmark" />
        </a>
        <button id="side-btn" className="panel-btn" type="button" popoverTarget="aux" aria-label="contents">
          <Glyph name="ring" cls="glyph" />
          <Glyph name="cross" cls="glyph swap" />
        </button>
      </header>
      <div className="layout">
        <Menu site={site} route={route} />
        <main>{children}</main>
        <Aside toc={toc} places={PLACES} socials={SOCIALS}>
          {cart && <Cart />}
        </Aside>
      </div>
      <Footer />
    </>
  )
}
