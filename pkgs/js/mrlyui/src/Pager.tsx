import { Symbol } from "./Glyphs"

export function Pager({ current, total, onPrev, onNext }: {
  current: number
  total: number
  onPrev: () => void
  onNext: () => void
}) {
  return (
    <nav className="pager" aria-label="Pagination">
      <button type="button" className="pager-step" onClick={onPrev} disabled={current <= 1} aria-label="Previous">
        <Symbol name="chevron_left" />
      </button>
      <span className="pager-count" aria-live="polite">
        {current} / {total}
      </span>
      <button type="button" className="pager-step" onClick={onNext} disabled={current >= total} aria-label="Next">
        <Symbol name="chevron_right" />
      </button>
    </nav>
  )
}
