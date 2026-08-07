import { cx } from "./lib"

export function Skeleton({ lines = 3, head = false, block = false }: {
  lines?: number
  head?: boolean
  block?: boolean
}) {
  return (
    <div className="skeleton" aria-hidden="true">
      {head && <span className="bar head" />}
      {Array.from({ length: lines }, (_, i) => (
        <span key={i} className={cx("bar", i + 1 === lines && "short")} />
      ))}
      {block && <span className="bar block" />}
    </div>
  )
}
