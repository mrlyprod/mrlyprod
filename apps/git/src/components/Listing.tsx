import { Symbol, seti } from "mrlyui"
import type { Dir } from "../lib/repo"
import { join } from "../lib/repo"
import { RAW } from "../lib/site"
import { href } from "../lib/tree"

export function Listing({ route, dir }: { route: string; dir: Dir }) {
  if (dir.subdirs.length === 0 && dir.files.length === 0 && dir.assets.length === 0) return null
  return (
    <nav className="tree" aria-label="contents">
      {dir.subdirs.map(sub => (
        <a className="leaf" key={sub} href={href(join(route, sub))}>
          <Symbol name="chevron_right" />
          {sub}
        </a>
      ))}
      {dir.files.map(([name, target]) => (
        <a className="leaf" key={name} href={href(target)}>
          <span className={seti(name)} aria-hidden="true" />
          {name}
        </a>
      ))}
      {dir.assets.map(([name, path]) => (
        <a className="leaf" key={name} href={`${RAW}${path}`}>
          <span className={seti(name)} aria-hidden="true" />
          {name}
        </a>
      ))}
    </nav>
  )
}
