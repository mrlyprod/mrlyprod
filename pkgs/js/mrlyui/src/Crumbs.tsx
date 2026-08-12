import { Fragment } from "react"
import type { ReactNode } from "react"

export function Crumbs({ root, route, children }: {
  root: string
  route: string
  children?: ReactNode
}) {
  const segs = route === "" ? [] : route.split("/")
  return (
    <nav className="crumbs" aria-label="Breadcrumb">
      {route === "" ? root : <a href="/">{root}</a>}
      {segs.map((seg, i) => (
        <Fragment key={seg + String(i)}>
          {" / "}
          {i + 1 === segs.length ? seg : <a href={`/${segs.slice(0, i + 1).join("/")}`}>{seg}</a>}
        </Fragment>
      ))}
      {children}
    </nav>
  )
}
