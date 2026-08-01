import { Shell } from "../components/Shell"
import type { Ctx } from "../lib/repo"

export function NotFound({ ctx }: { ctx: Ctx }) {
  return (
    <Shell ctx={ctx} route="404" current=" " title="nothing here" desc="nothing here">
      <h1>nothing here</h1>
      <p>
        <a href="/">home</a>
      </p>
    </Shell>
  )
}
