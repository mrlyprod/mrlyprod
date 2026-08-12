import type { Site } from "../lib/data"
import { menuLinks } from "../lib/site"

export function Menu({ site, route }: { site: Site; route: string }) {
  return (
    <nav className="tree" aria-label="menu">
      {menuLinks(route, site).map(link => (
        <a key={link.href} className={link.cls} href={link.href} aria-current={link.current ? "page" : undefined}>
          {link.name}
        </a>
      ))}
    </nav>
  )
}
