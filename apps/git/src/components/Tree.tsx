import { useMemo } from "react"
import type { Ctx } from "../lib/repo"
import { explorer } from "../lib/tree"

export function Tree({ ctx, current }: { ctx: Ctx; current: string }) {
  const html = useMemo(() => explorer(ctx, current), [ctx, current])
  return <nav className="tree" aria-label="files" dangerouslySetInnerHTML={{ __html: html }} />
}
