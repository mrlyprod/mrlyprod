import { useEffect, useId, useState } from "react"
import type { KeyboardEvent, ReactNode } from "react"
import { cx } from "./lib"
import { Popover } from "./Popover"
import { Symbol } from "./Glyphs"
import type { SymbolName } from "./Glyphs"

export function Dropdown<T extends string>({
  label,
  items,
  onSelect,
  align = "left",
}: {
  label: ReactNode
  items: { label: string; value: T; icon?: SymbolName; danger?: boolean }[]
  onSelect: (value: T) => void
  align?: "left" | "right"
}) {
  const id = useId()
  const [active, setActive] = useState(0)

  useEffect(() => {
    document.getElementById(`${id}-${active}`)?.scrollIntoView({ block: "nearest" })
  }, [id, active])

  return (
    <Popover
      align={align}
      trigger={({ open, toggle }) => {
        const lift = () => {
          setActive(0)
          toggle()
        }
        const pick = () => {
          const item = items[active]
          if (!item) return
          onSelect(item.value)
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
            setActive((held) => Math.min(held + 1, items.length - 1))
          } else if (event.key === "ArrowUp") {
            event.preventDefault()
            setActive((held) => Math.max(held - 1, 0))
          } else if (event.key === "Home") {
            event.preventDefault()
            setActive(0)
          } else if (event.key === "End") {
            event.preventDefault()
            setActive(items.length - 1)
          } else if (event.key === "Enter" || event.key === " ") {
            event.preventDefault()
            pick()
          }
        }
        return (
          <button
            type="button"
            className={cx("dropdown-trigger", open && "open")}
            aria-haspopup="menu"
            aria-expanded={open}
            aria-controls={open ? `${id}-menu` : undefined}
            aria-activedescendant={open ? `${id}-${active}` : undefined}
            onClick={lift}
            onKeyDown={keys}
          >
            {label}
            <span className="dropdown-chevron">
              <Symbol name="keyboard_arrow_down" size="var(--font-lg)" />
            </span>
          </button>
        )
      }}
    >
      {(close) => (
        <div className="menu" role="menu" id={`${id}-menu`}>
          {items.map((item, index) => (
            <button
              key={item.value}
              type="button"
              role="menuitem"
              id={`${id}-${index}`}
              tabIndex={-1}
              className={cx("menu-item", item.danger && "danger", index === active && "active")}
              onMouseEnter={() => setActive(index)}
              onClick={() => {
                onSelect(item.value)
                close()
              }}
            >
              {item.icon && (
                <span className="menu-item-icon">
                  <Symbol name={item.icon} size="var(--font-size)" />
                </span>
              )}
              <span className="menu-item-label">{item.label}</span>
            </button>
          ))}
        </div>
      )}
    </Popover>
  )
}
