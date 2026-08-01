import type { ReactNode } from "react"
import { Aside, Footer, Glyph, Letters, useHead } from "mrlydom"
import type { Section } from "mrlydom"
import { Grip } from "./Grip"
import { Panel } from "./Panel"
import { usePanes } from "../lib/panes"
import type { Ctx } from "../lib/repo"
import { BASE, CLONE, PLACES, ROOT, SOCIALS } from "../lib/site"

type Props = {
  ctx: Ctx
  route: string
  current?: string
  title: string
  desc: string
  toc?: Section[]
  status?: ReactNode
  children: ReactNode
}

function knob(open: boolean): string {
  return open ? "panel-btn dock open" : "panel-btn dock"
}

// SHELL

export function Shell({ ctx, route, current = route, title, desc, toc = [], status, children }: Props) {
  useHead({ route, title, desc, root: ROOT, base: BASE })
  const { wide, left, right } = usePanes()
  return (
    <>
      <header>
        <button
          id="tree-btn"
          className={knob(left.open)}
          type="button"
          aria-label="explorer"
          aria-expanded={left.open}
          aria-controls="explorer"
          onClick={left.toggle}
        >
          <Glyph name="plus" cls="glyph" />
          <Glyph name="minus" cls="glyph swap" />
        </button>
        <a className="lettermark" href="/" aria-label="surprise me" title="surprise me">
          <Letters cls="headmark" />
        </a>
        <button
          id="side-btn"
          className={knob(right.open)}
          type="button"
          aria-label="contents"
          aria-expanded={right.open}
          aria-controls="aux"
          onClick={right.toggle}
        >
          <Glyph name="ring" cls="glyph" />
          <Glyph name="cross" cls="glyph swap" />
        </button>
      </header>
      <div className="layout dock">
        <Panel ctx={ctx} current={current} pane={left} shut={wide && !left.open} />
        {wide && left.open && <Grip side="left" pane={left} />}
        <main>{children}</main>
        {wide && right.open && <Grip side="right" pane={right} />}
        <Aside toc={toc} places={PLACES} socials={SOCIALS} shut={wide && !right.open} ref={right.hold} />
      </div>
      <Footer>
        <code className="clone">{CLONE}</code>
      </Footer>
      {status}
    </>
  )
}
