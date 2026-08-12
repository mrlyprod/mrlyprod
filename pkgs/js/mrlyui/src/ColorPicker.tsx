import type { CSSProperties } from "react"
import { cx } from "./lib"
import { POOL, css } from "./colors"
import type { ColorName } from "./colors"

// SWATCH

export type Swatch<T extends string> = { name: T; color: string }

function pool(): Swatch<ColorName>[] {
  return POOL.map(name => ({ name, color: css(name) }))
}

// PICKER

export function ColorPicker<T extends string = ColorName>({ swatches, value = null, onChange, auto, big }: {
  swatches?: Swatch<T>[]
  value?: T | T[] | null
  onChange: (color: T | null) => void
  auto?: boolean
  big?: boolean
}) {
  const tiles = swatches ?? (pool() as unknown as Swatch<T>[])
  const picked = Array.isArray(value) ? value : value === null ? [] : [value]
  const bare = Array.isArray(value) ? value.length === 0 : value === null
  return (
    <div className={cx("swatches", big && "big")}>
      {auto === true && (
        <button
          type="button"
          className={cx("swatch", "auto", bare && "active")}
          aria-pressed={bare}
          onClick={() => onChange(null)}
        >
          AUTO
        </button>
      )}
      {tiles.map(tile => {
        const on = picked.includes(tile.name)
        return (
          <button
            key={tile.name}
            type="button"
            className={cx("swatch", on && "active")}
            style={{ "--swatch-color": tile.color } as CSSProperties}
            aria-label={tile.name}
            aria-pressed={on}
            title={tile.name}
            onClick={() => onChange(tile.name)}
          />
        )
      })}
    </div>
  )
}
