import { useEffect, useId, useRef, useState } from "react"
import type { KeyboardEvent, ReactNode } from "react"
import { cx } from "./lib"
import { Search } from "./Search"

export function Autocomplete<T>({
  value,
  onChange,
  items,
  itemKey,
  renderItem,
  onSelect,
  placeholder = "Search",
  emptyLabel = "No matches",
}: {
  value: string
  onChange: (value: string) => void
  items: T[]
  itemKey: (item: T) => string
  renderItem: (item: T, active: boolean) => ReactNode
  onSelect: (item: T) => void
  placeholder?: string
  emptyLabel?: string
}) {
  const id = useId()
  const [open, setOpen] = useState(false)
  const [active, setActive] = useState(0)
  const anchor = useRef<HTMLDivElement>(null)
  const show = open && value.trim().length > 0

  useEffect(() => {
    setActive(0)
  }, [value])

  useEffect(() => {
    if (!open) return
    const down = (event: PointerEvent): void => {
      const node = event.target
      if (!(node instanceof Node)) return
      if (anchor.current?.contains(node) === true) return
      setOpen(false)
    }
    document.addEventListener("pointerdown", down)
    return () => document.removeEventListener("pointerdown", down)
  }, [open])

  useEffect(() => {
    if (show) document.getElementById(`${id}-${active}`)?.scrollIntoView({ block: "nearest" })
  }, [id, active, show])

  const pick = (item: T) => {
    onSelect(item)
    setOpen(false)
  }

  const keys = (event: KeyboardEvent<HTMLDivElement>) => {
    if (!show) return
    if (event.key === "ArrowDown") {
      event.preventDefault()
      setActive((held) => Math.min(held + 1, items.length - 1))
    } else if (event.key === "ArrowUp") {
      event.preventDefault()
      setActive((held) => Math.max(held - 1, 0))
    } else if (event.key === "Enter") {
      const item = items[active]
      if (item === undefined) return
      event.preventDefault()
      pick(item)
    } else if (event.key === "Escape") {
      event.stopPropagation()
      setOpen(false)
    }
  }

  return (
    <div
      className="autocomplete"
      ref={anchor}
      onKeyDown={keys}
      onFocusCapture={() => setOpen(true)}
    >
      <Search value={value} onChange={onChange} placeholder={placeholder} />
      {show && (
        <div className="autocomplete-panel" role="listbox" id={`${id}-list`}>
          {items.length === 0 ? (
            <div className="autocomplete-empty">{emptyLabel}</div>
          ) : (
            items.map((item, index) => (
              <button
                key={itemKey(item)}
                type="button"
                role="option"
                id={`${id}-${index}`}
                tabIndex={-1}
                aria-selected={index === active}
                className={cx("menu-item", "autocomplete-item", index === active && "active")}
                onMouseEnter={() => setActive(index)}
                onClick={() => pick(item)}
              >
                {renderItem(item, index === active)}
              </button>
            ))
          )}
        </div>
      )}
    </div>
  )
}

export function highlight(text: string, query: string): ReactNode {
  const needle = query.trim()
  if (needle === "") return text
  const start = text.toLowerCase().indexOf(needle.toLowerCase())
  if (start < 0) return text
  const end = start + needle.length
  return (
    <>
      {text.slice(0, start)}
      <mark className="hl">{text.slice(start, end)}</mark>
      {text.slice(end)}
    </>
  )
}
