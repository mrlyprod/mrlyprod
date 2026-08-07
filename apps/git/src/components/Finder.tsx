import { useMemo, useState } from "react"
import { Caption, Search, Stack, Text, seti } from "mrlyui"
import { hunt } from "../lib/find"
import type { Ctx } from "../lib/repo"
import { href } from "../lib/tree"

// MARK

function Mark({ text, query }: { text: string; query: string }) {
  const at = query === "" ? -1 : text.toLowerCase().indexOf(query.toLowerCase())
  if (at === -1) return <>{text}</>
  return (
    <>
      {text.slice(0, at)}
      <mark className="hl">{text.slice(at, at + query.length)}</mark>
      {text.slice(at + query.length)}
    </>
  )
}

// FINDER

export function Finder({ ctx }: { ctx: Ctx }) {
  const [query, set] = useState("")
  const needle = query.trim()
  const hits = useMemo(() => hunt(ctx, needle), [ctx, needle])
  return (
    <Stack tight>
      <Search value={query} onChange={set} placeholder="search" autoFocus />
      {needle !== "" && hits.length === 0 && <Text className="caption">nothing found</Text>}
      <nav className="tree" aria-label="results">
        {hits.map(hit => (
          <a className="leaf" key={hit.route} href={href(hit.route)}>
            <span className={seti(hit.name)} aria-hidden="true" />
            <Stack tight>
              <span>
                <Mark text={hit.name} query={needle} />
              </span>
              <Caption>
                <Mark text={hit.hint} query={needle} />
              </Caption>
            </Stack>
          </a>
        ))}
      </nav>
    </Stack>
  )
}
