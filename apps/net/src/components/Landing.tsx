import type { Site } from "../lib/data"
import { FUNNEL } from "../lib/site"

export function Landing({ site }: { site: Site }) {
  return (
    <>
      <p className="lede">{site.home[1]}</p>
      <nav className="funnel" aria-label="start">
        {FUNNEL.map(([name, href]) => (
          <a key={name} href={href}>
            {name}
          </a>
        ))}
      </nav>
      <section className="cards" aria-label="products">
        {Object.entries(site.products).map(([route, [title, desc]]) => (
          <a key={route} href={`/${route}`}>
            <strong>{title}</strong>
            <span>{desc}</span>
          </a>
        ))}
      </section>
    </>
  )
}
