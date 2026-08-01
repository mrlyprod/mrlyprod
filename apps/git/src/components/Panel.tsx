import { useState } from "react"
import { Search } from "./Search"
import { Tree } from "./Tree"
import type { Pane } from "../lib/panes"
import type { Ctx } from "../lib/repo"

const TABS: [string, string, string][] = [
  ["tree", "files", "account_tree"],
  ["find", "search", "search"],
]

type Props = { ctx: Ctx; current: string; pane: Pane; shut: boolean }

export function Panel({ ctx, current, pane, shut }: Props) {
  const [tab, set] = useState("tree")
  return (
    <div
      className={shut ? "pane left shut" : "pane left"}
      id="explorer"
      popover="auto"
      aria-label="explorer"
      ref={node => {
        pane.hold(node)
      }}
    >
      <div className="tabs" role="tablist" aria-label="explorer">
        {TABS.map(([name, label, icon]) => (
          <button
            className="tab"
            key={name}
            type="button"
            role="tab"
            aria-selected={tab === name}
            aria-label={label}
            title={label}
            onClick={() => set(name)}
          >
            <span className="ic" aria-hidden="true">
              {icon}
            </span>
          </button>
        ))}
      </div>
      {tab === "tree" ? <Tree ctx={ctx} current={current} /> : <Search ctx={ctx} />}
    </div>
  )
}
