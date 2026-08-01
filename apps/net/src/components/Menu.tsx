import type { Site } from "../lib/data"
import { menuLinks } from "../lib/site"

type Props = { site: Site; route: string; sheet?: boolean }

export function Menu({ site, route, sheet = false }: Props) {
  return (
    <nav
      className={sheet ? "menu sheet" : "menu pane left"}
      id={sheet ? undefined : "explorer"}
      popover={sheet ? undefined : "auto"}
      aria-label={sheet ? "pages" : "menu"}
    >
      {menuLinks(route, site).map(link => (
        <a key={link.href} className={link.cls} href={link.href} aria-current={link.current ? "page" : undefined}>
          {link.name}
        </a>
      ))}
    </nav>
  )
}
