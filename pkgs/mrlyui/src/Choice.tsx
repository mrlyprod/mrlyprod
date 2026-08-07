import { cx } from "./lib"
import { Radio } from "./Radio"
import { Select } from "./Select"

// CHOICE

export function Choice<T extends string>({ options, value, onChange, mode = "select", label, disabled }: {
  options: { label: string; value: T }[]
  value: T
  onChange: (value: T) => void
  mode?: "row" | "cycle" | "select"
  label?: string
  disabled?: boolean
}) {
  if (mode === "cycle") {
    const index = options.findIndex(option => option.value === value)
    const chosen = index >= 0 ? options[index] : undefined
    const reading = chosen?.label ?? value
    return (
      <button
        type="button"
        className="button cycle"
        disabled={disabled === true || options.length === 0}
        onClick={() => {
          const next = options[(index + 1 + options.length) % options.length]
          if (next !== undefined) onChange(next.value)
        }}
      >
        {label === undefined ? reading : `${label}: ${reading}`}
      </button>
    )
  }
  return (
    <div className={cx("choice", mode)}>
      {label !== undefined && <span className="choice-label">{label}</span>}
      {mode === "row" ? (
        <Radio options={options} value={value} onChange={onChange} disabled={disabled} />
      ) : (
        <Select options={options} value={value} onChange={onChange} disabled={disabled} />
      )}
    </div>
  )
}
