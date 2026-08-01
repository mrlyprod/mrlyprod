import { useEffect, useMemo, useRef, useState } from "react"
import { hunt } from "../lib/find"
import type { Ctx } from "../lib/repo"
import { kind } from "../lib/tree"

// MARK

function Mark({ text, query }: { text: string; query: string }) {
  const at = query === "" ? -1 : text.toLowerCase().indexOf(query.toLowerCase())
  if (at === -1) return <>{text}</>
  return (
    <>
      {text.slice(0, at)}
      <mark>{text.slice(at, at + query.length)}</mark>
      {text.slice(at + query.length)}
    </>
  )
}

// SEARCH

export function Search({ ctx }: { ctx: Ctx }) {
  const [query, set] = useState("")
  const box = useRef<HTMLInputElement>(null)
  useEffect(() => {
    box.current?.focus()
  }, [])
  const needle = query.trim()
  const hits = useMemo(() => hunt(ctx, needle), [ctx, needle])
  return (
    <div className="tree find">
      <input
        ref={box}
        className="probe"
        type="search"
        value={query}
        placeholder="search"
        aria-label="search"
        autoComplete="off"
        spellCheck={false}
        onChange={event => set(event.target.value)}
      />
      {needle !== "" && hits.length === 0 && <p className="none">nothing found</p>}
      {hits.map(hit => (
        <a className="leaf" key={hit.route} href={hit.route === "" ? "/" : `/${hit.route}`}>
          <span className={`si${kind(hit.name)}`} aria-hidden="true" />
          <span className="row">
            <span className="name">
              <Mark text={hit.name} query={needle} />
            </span>
            <span className="hint">
              <Mark text={hit.hint} query={needle} />
            </span>
          </span>
        </a>
      ))}
    </div>
  )
}
