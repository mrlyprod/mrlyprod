import { useEffect, useRef, useState } from "react"
import type { KeyboardEvent } from "react"
import { cx } from "./lib"
import { Popover } from "./Popover"
import { Symbol } from "./Glyphs"

const WEEKDAYS = ["S", "M", "T", "W", "T", "F", "S"]

function sameDay(a: Date, b: Date): boolean {
  return (
    a.getFullYear() === b.getFullYear() &&
    a.getMonth() === b.getMonth() &&
    a.getDate() === b.getDate()
  )
}

export function Calendar({ value, onSelect }: {
  value?: Date
  onSelect: (date: Date) => void
}) {
  const today = new Date()
  const seed = value ?? today
  const [view, setView] = useState(() => new Date(seed.getFullYear(), seed.getMonth(), 1))
  const [focus, setFocus] = useState(() => new Date(seed.getFullYear(), seed.getMonth(), seed.getDate()))
  const grid = useRef<HTMLDivElement>(null)
  const roving = useRef(false)

  const year = view.getFullYear()
  const month = view.getMonth()
  const offset = new Date(year, month, 1).getDay()
  const days = new Date(year, month + 1, 0).getDate()
  const focusDay = focus.getFullYear() === year && focus.getMonth() === month ? focus.getDate() : 1

  useEffect(() => {
    if (!roving.current) return
    roving.current = false
    grid.current?.querySelector<HTMLButtonElement>("[tabindex='0']")?.focus()
  }, [focus])

  const shift = (delta: number) => {
    setView(new Date(year, month + delta, 1))
  }

  const leap = (delta: number) => {
    setView(new Date(year + delta, month, 1))
  }

  const rove = (delta: number) => {
    const next = new Date(focus.getFullYear(), focus.getMonth(), focus.getDate() + delta)
    roving.current = true
    setFocus(next)
    setView(new Date(next.getFullYear(), next.getMonth(), 1))
  }

  const keys = (event: KeyboardEvent<HTMLDivElement>) => {
    if (event.key === "ArrowLeft") {
      event.preventDefault()
      rove(-1)
    } else if (event.key === "ArrowRight") {
      event.preventDefault()
      rove(1)
    } else if (event.key === "ArrowUp") {
      event.preventDefault()
      rove(-7)
    } else if (event.key === "ArrowDown") {
      event.preventDefault()
      rove(7)
    }
  }

  return (
    <div className="calendar">
      <div className="calendar-header">
        <button
          type="button"
          className="calendar-nav"
          aria-label="Previous year"
          onClick={() => leap(-1)}
        >
          <Symbol name="keyboard_double_arrow_left" size="var(--font-lg)" />
        </button>
        <button
          type="button"
          className="calendar-nav"
          aria-label="Previous month"
          onClick={() => shift(-1)}
        >
          <Symbol name="chevron_left" size="var(--font-lg)" />
        </button>
        <span className="calendar-title" aria-live="polite">
          {view.toLocaleDateString(undefined, { month: "long", year: "numeric" })}
        </span>
        <button
          type="button"
          className="calendar-nav"
          aria-label="Next month"
          onClick={() => shift(1)}
        >
          <Symbol name="chevron_right" size="var(--font-lg)" />
        </button>
        <button
          type="button"
          className="calendar-nav"
          aria-label="Next year"
          onClick={() => leap(1)}
        >
          <Symbol name="keyboard_double_arrow_right" size="var(--font-lg)" />
        </button>
      </div>
      <div className="calendar-grid" ref={grid} onKeyDown={keys}>
        {WEEKDAYS.map((letter, index) => (
          <span key={index} className="calendar-weekday" aria-hidden="true">
            {letter}
          </span>
        ))}
        {Array.from({ length: offset }, (_, index) => (
          <span key={`pad-${index}`} className="calendar-day empty" />
        ))}
        {Array.from({ length: days }, (_, index) => {
          const day = index + 1
          const date = new Date(year, month, day)
          const picked = value !== undefined && sameDay(date, value)
          return (
            <button
              key={day}
              type="button"
              tabIndex={day === focusDay ? 0 : -1}
              aria-label={date.toLocaleDateString(undefined, { year: "numeric", month: "long", day: "numeric" })}
              aria-pressed={picked}
              aria-current={sameDay(date, today) ? "date" : undefined}
              className={cx("calendar-day", picked && "selected", sameDay(date, today) && "today")}
              onClick={() => {
                setFocus(date)
                onSelect(date)
              }}
            >
              {day}
            </button>
          )
        })}
      </div>
    </div>
  )
}

function iso(date: Date): string {
  const month = String(date.getMonth() + 1).padStart(2, "0")
  const day = String(date.getDate()).padStart(2, "0")
  return `${String(date.getFullYear())}-${month}-${day}`
}

function parse(text: string): Date | null {
  const clean = text.trim()
  if (clean === "") return null
  const exact = /^(\d{4})-(\d{1,2})-(\d{1,2})$/.exec(clean)
  if (exact) {
    const parsed = new Date(Number(exact[1]), Number(exact[2]) - 1, Number(exact[3]))
    return Number.isNaN(parsed.getTime()) ? null : parsed
  }
  const loose = new Date(clean)
  return Number.isNaN(loose.getTime()) ? null : loose
}

export function DatePicker({ value, onChange, placeholder = "yyyy-mm-dd" }: {
  value?: Date
  onChange: (date: Date) => void
  placeholder?: string
}) {
  const [text, setText] = useState(() => (value ? iso(value) : ""))
  const held = useRef(value ? iso(value) : "")

  useEffect(() => {
    const next = value ? iso(value) : ""
    if (next !== held.current) {
      held.current = next
      setText(next)
    }
  }, [value])

  const commit = () => {
    const parsed = parse(text)
    if (parsed !== null) {
      held.current = iso(parsed)
      setText(held.current)
      onChange(parsed)
    }
  }

  return (
    <div className="datepicker">
      <input
        className="input"
        value={text}
        placeholder={placeholder}
        onChange={event => setText(event.target.value)}
        onBlur={commit}
        onKeyDown={event => {
          if (event.key === "Enter") commit()
        }}
      />
      <Popover
        trigger={({ open, toggle }) => (
          <button
            type="button"
            className={cx("picker-button", open && "open")}
            aria-label="Open calendar"
            aria-haspopup="dialog"
            aria-expanded={open}
            onClick={toggle}
          >
            <Symbol name="calendar_month" size="var(--font-lg)" />
          </button>
        )}
        align="right"
      >
        {(close) => (
          <Calendar
            value={value}
            onSelect={(date) => {
              onChange(date)
              close()
            }}
          />
        )}
      </Popover>
    </div>
  )
}
