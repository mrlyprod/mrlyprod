import { Shell } from "../components/Shell"
import type { Site } from "../lib/data"

export function NotFound({ site }: { site: Site }) {
  return (
    <Shell site={site} route="404" title="nothing here" desc="nothing here" cart={false}>
      <h1>nothing here</h1>
      <p>
        <a href="/">home</a>
      </p>
    </Shell>
  )
}
