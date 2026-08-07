import type { CSSProperties } from "react"
import { cx } from "./lib"

// NAMES

export type SymbolName =
  | "account_tree"
  | "alternate_email"
  | "brightness_auto"
  | "check"
  | "chevron_left"
  | "chevron_right"
  | "close"
  | "content_copy"
  | "custom_typography"
  | "dark_mode"
  | "download"
  | "explore"
  | "gavel"
  | "grid_view"
  | "groups"
  | "home"
  | "info"
  | "keyboard_arrow_down"
  | "keyboard_arrow_up"
  | "light_mode"
  | "list"
  | "mail"
  | "markdown"
  | "more_vert"
  | "pause"
  | "play_arrow"
  | "raw_on"
  | "refresh"
  | "search"
  | "settings"
  | "shield"
  | "toc"
  | "volunteer_activism"

// SIZE

function sized(name: string, size: number | string | undefined): CSSProperties | undefined {
  if (size === undefined) return undefined
  return { [name]: typeof size === "number" ? `${size}px` : size } as CSSProperties
}

// SYMBOL

export function Symbol({ name, size }: {
  name: SymbolName
  size?: number | string
}) {
  return (
    <span className="symbol" aria-hidden="true" style={sized("--symbol-size", size)}>
      {name}
    </span>
  )
}

// EMOJI

export function Emoji({ glyph, label, size }: {
  glyph: string
  label?: string
  size?: number | string
}) {
  return (
    <span
      className="emoji"
      role={label ? "img" : undefined}
      aria-label={label}
      aria-hidden={label ? undefined : true}
      style={sized("--emoji-size", size)}
    >
      {glyph}
    </span>
  )
}

// ICON

export function Icon({ emoji, label, active, onClick }: {
  emoji: string
  label?: string
  active?: boolean
  onClick?: () => void
}) {
  const skin = cx("icon", active && "active")
  const tile = (
    <span className="icon-tile" aria-hidden="true">
      <span className="emoji">{emoji}</span>
    </span>
  )
  if (onClick) {
    return (
      <button type="button" className={skin} onClick={onClick} aria-label={label ?? emoji}>
        {tile}
        {label && <span className="icon-label">{label}</span>}
      </button>
    )
  }
  return (
    <span className={skin}>
      {tile}
      {label && <span className="icon-label">{label}</span>}
    </span>
  )
}
