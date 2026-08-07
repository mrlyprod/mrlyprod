import { useRef } from "react"
import type { KeyboardEvent } from "react"
import { cx } from "./lib"

export function Tabs<T extends string>({ tabs, value, onChange }: {
  tabs: { label: string; value: T }[]
  value: T
  onChange: (value: T) => void
}) {
  const ref = useRef<HTMLDivElement>(null)
  const move = (event: KeyboardEvent<HTMLDivElement>) => {
    const step = event.key === "ArrowRight" ? 1 : event.key === "ArrowLeft" ? -1 : 0
    if (step === 0 && event.key !== "Home" && event.key !== "End") return
    event.preventDefault()
    const index = tabs.findIndex((tab) => tab.value === value)
    const next =
      event.key === "Home" ? 0 :
      event.key === "End" ? tabs.length - 1 :
      (index + step + tabs.length) % tabs.length
    const tab = tabs[next]
    if (!tab) return
    onChange(tab.value)
    ref.current?.querySelectorAll<HTMLButtonElement>("[role=tab]")[next]?.focus()
  }
  return (
    <div ref={ref} className="tabs" role="tablist" onKeyDown={move}>
      {tabs.map((tab) => (
        <button
          key={tab.value}
          type="button"
          role="tab"
          aria-selected={value === tab.value}
          tabIndex={value === tab.value ? 0 : -1}
          className={cx("tab", value === tab.value && "active")}
          onClick={() => onChange(tab.value)}
        >
          {tab.label}
        </button>
      ))}
    </div>
  )
}
