import { Button, Cluster, Title } from "mrlyui"
import { Shell } from "../components/Shell"
import type { Site } from "../lib/data"

export function NotFound({ site }: { site: Site }) {
  return (
    <Shell site={site} route="404" title="nothing here" desc="nothing here">
      <Title>nothing here</Title>
      <Cluster>
        <Button href="/">home</Button>
      </Cluster>
    </Shell>
  )
}
