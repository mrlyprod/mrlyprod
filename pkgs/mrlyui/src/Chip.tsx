import type { KeyboardEvent, ReactNode } from "react"
import { Symbol } from "./Glyphs"
import { cx } from "./lib"

export function Chip({ children, active = false, onClick, onRemove }: {
  children: ReactNode
  active?: boolean
  onClick?: () => void
  onRemove?: () => void
}) {
  const press = onClick
    ? (event: KeyboardEvent<HTMLSpanElement>) => {
        if (event.key !== "Enter" && event.key !== " ") return
        event.preventDefault()
        onClick()
      }
    : undefined
  return (
    <span
      className={cx("chip", active && "active", onClick && "clickable")}
      role={onClick ? "button" : undefined}
      tabIndex={onClick ? 0 : undefined}
      aria-pressed={onClick ? active : undefined}
      onClick={onClick}
      onKeyDown={press}
    >
      {children}
      {onRemove && (
        <button
          type="button"
          className="chip-remove"
          aria-label="Remove"
          onClick={(event) => {
            event.stopPropagation()
            onRemove()
          }}
        >
          <Symbol name="close" size="var(--font-sm)" />
        </button>
      )}
    </span>
  )
}
