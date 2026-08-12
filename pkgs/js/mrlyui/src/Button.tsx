import { useRef } from "react"
import type { CSSProperties, ReactNode } from "react"
import { cx } from "./lib"

// BUTTON

export function Button({ children, href, onClick, onPress, onLift, active, disabled, primary, wide, big, bg, className }: {
  children: ReactNode
  href?: string
  onClick?: () => void
  onPress?: () => void
  onLift?: () => void
  active?: boolean
  disabled?: boolean
  primary?: boolean
  wide?: boolean
  big?: boolean
  bg?: string
  className?: string
}) {
  const down = useRef(false)
  const skin = cx("button", active && "active", primary && "primary", wide && "wide", big && "big", className)
  const paint: CSSProperties | undefined = bg === undefined ? undefined : { backgroundColor: bg }
  if (href !== undefined) {
    return (
      <a
        href={href}
        className={cx(skin, disabled && "disabled")}
        style={paint}
        aria-disabled={disabled}
        onClick={event => {
          if (disabled === true) {
            event.preventDefault()
            return
          }
          onClick?.()
        }}
      >
        {children}
      </a>
    )
  }
  const held = onPress !== undefined || onLift !== undefined
  const lift = () => {
    if (!down.current) return
    down.current = false
    onLift?.()
  }
  return (
    <button
      type="button"
      className={skin}
      style={held ? { ...paint, touchAction: "none" } : paint}
      onClick={onPress === undefined ? onClick : undefined}
      onPointerDown={
        onPress === undefined
          ? undefined
          : () => {
              down.current = true
              onPress()
            }
      }
      onPointerUp={held ? lift : undefined}
      onPointerLeave={held ? lift : undefined}
      onPointerCancel={held ? lift : undefined}
      disabled={disabled}
    >
      {children}
    </button>
  )
}
