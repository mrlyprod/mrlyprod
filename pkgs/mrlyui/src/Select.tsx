import { useEffect, useId, useRef, useState } from "react"
import type { KeyboardEvent } from "react"
import { cx } from "./lib"
import { Popover } from "./Popover"
import { Symbol } from "./Glyphs"

export function Select<T extends string>({
  options,
  value,
  onChange,
  placeholder = "Select",
  disabled = false,
}: {
  options: { label: string; value: T }[]
  value?: T
  onChange: (value: T) => void
  placeholder?: string
  disabled?: boolean
}) {
  const id = useId()
  const [active, setActive] = useState(0)
  const buffer = useRef("")
  const timer = useRef(0)
  const selected = options.findIndex((option) => option.value === value)
  const chosen = selected >= 0 ? options[selected] : undefined

  useEffect(() => {
    document.getElementById(`${id}-${active}`)?.scrollIntoView({ block: "nearest" })
  }, [id, active])

  useEffect(() => () => window.clearTimeout(timer.current), [])

  const seek = (key: string) => {
    window.clearTimeout(timer.current)
    timer.current = window.setTimeout(() => {
      buffer.current = ""
    }, 500)
    buffer.current += key.toLowerCase()
    const hit = options.findIndex((option) => option.label.toLowerCase().startsWith(buffer.current))
    if (hit >= 0) setActive(hit)
  }

  return (
    <Popover
      trigger={({ open, toggle }) => {
        const lift = () => {
          setActive(selected >= 0 ? selected : 0)
          toggle()
        }
        const pick = () => {
          const option = options[active]
          if (!option) return
          onChange(option.value)
          toggle()
        }
        const keys = (event: KeyboardEvent<HTMLButtonElement>) => {
          if (!open) {
            if (event.key === "ArrowDown" || event.key === "ArrowUp") {
              event.preventDefault()
              lift()
            }
            return
          }
          if (event.key === "ArrowDown") {
            event.preventDefault()
            setActive((held) => Math.min(held + 1, options.length - 1))
          } else if (event.key === "ArrowUp") {
            event.preventDefault()
            setActive((held) => Math.max(held - 1, 0))
          } else if (event.key === "Home") {
            event.preventDefault()
            setActive(0)
          } else if (event.key === "End") {
            event.preventDefault()
            setActive(options.length - 1)
          } else if (event.key === "Enter" || event.key === " ") {
            event.preventDefault()
            pick()
          } else if (event.key.length === 1 && event.key !== " ") {
            seek(event.key)
          }
        }
        return (
          <button
            type="button"
            className={cx("select-trigger", open && "open")}
            disabled={disabled}
            aria-haspopup="listbox"
            aria-expanded={open}
            aria-controls={open ? `${id}-list` : undefined}
            aria-activedescendant={open ? `${id}-${active}` : undefined}
            onClick={lift}
            onKeyDown={keys}
          >
            <span className={cx("select-value", !chosen && "placeholder")}>
              {chosen ? chosen.label : placeholder}
            </span>
            <span className="select-chevron">
              <Symbol name="keyboard_arrow_down" size="var(--font-lg)" />
            </span>
          </button>
        )
      }}
    >
      {(close) => (
        <div className="menu" role="listbox" id={`${id}-list`}>
          {options.map((option, index) => (
            <button
              key={option.value}
              type="button"
              role="option"
              id={`${id}-${index}`}
              tabIndex={-1}
              aria-selected={option.value === value}
              className={cx(
                "menu-item",
                option.value === value && "selected",
                index === active && "active",
              )}
              onMouseEnter={() => setActive(index)}
              onClick={() => {
                onChange(option.value)
                close()
              }}
            >
              <span className="menu-item-label">{option.label}</span>
              {option.value === value && <Symbol name="check" size="var(--font-size)" />}
            </button>
          ))}
        </div>
      )}
    </Popover>
  )
}
