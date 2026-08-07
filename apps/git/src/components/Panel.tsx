import { useMemo, useState } from "react"
import type { ReactNode } from "react"
import { Stack, Symbol, Tabs, Tree } from "mrlyui"
import { Finder } from "./Finder"
import type { Ctx } from "../lib/repo"
import { ROOT } from "../lib/site"
import { explorer, href } from "../lib/tree"

type Tab = "tree" | "find"

const TABS: { label: ReactNode; value: Tab; title: string }[] = [
  { label: <Symbol name="account_tree" />, value: "tree", title: "files" },
  { label: <Symbol name="search" />, value: "find", title: "search" },
]

export function Panel({ ctx, current }: { ctx: Ctx; current?: string }) {
  const [tab, set] = useState<Tab>("tree")
  const nodes = useMemo(() => explorer(ctx), [ctx])
  return (
    <Stack tight>
      <Tabs tabs={TABS} value={tab} onChange={set} />
      {tab === "tree" ? (
        <Tree nodes={nodes} current={current === undefined ? "" : href(current)} root={{ name: ROOT, href: "/" }} />
      ) : (
        <Finder ctx={ctx} />
      )}
    </Stack>
  )
}
