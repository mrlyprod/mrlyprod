import { Text } from "mrlyui"
import { Shell } from "../components/Shell"
import type { Ctx } from "../lib/repo"

export function NotFound({ ctx }: { ctx: Ctx }) {
  return (
    <Shell ctx={ctx} route="404" title="nothing here" desc="nothing here">
      <h1 className="title">nothing here</h1>
      <Text>
        <a href="/">home</a>
      </Text>
    </Shell>
  )
}
