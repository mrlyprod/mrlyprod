import { useMemo } from "react"
import type { CSSProperties, ReactNode } from "react"
import { cx } from "./lib"
import { css, randomColor } from "./colors"
import type { ColorName } from "./colors"

// PLATE

function usePlate(plate: ColorName | "auto" | undefined): string | undefined {
  return useMemo(() => {
    if (!plate) return undefined
    return css(plate === "auto" ? randomColor() : plate)
  }, [plate])
}

function plated(style: CSSProperties | undefined, color: string | undefined): CSSProperties | undefined {
  if (!color) return style
  return { ...style, "--plate": color } as CSSProperties
}

// BOX

export function Box({ children, className, style, onClick, plate }: {
  children?: ReactNode
  className?: string
  style?: CSSProperties
  onClick?: () => void
  plate?: ColorName | "auto"
}) {
  const color = usePlate(plate)
  return (
    <div className={cx("box", className)} style={plated(style, color)} onClick={onClick}>
      {children}
    </div>
  )
}

// FLOW

export function Frame({ children, className, style }: {
  children?: ReactNode
  className?: string
  style?: CSSProperties
}) {
  return (
    <div className={cx("frame", className)} style={style}>
      {children}
    </div>
  )
}

export function Stack({ children, className, style, tight, airy }: {
  children?: ReactNode
  className?: string
  style?: CSSProperties
  tight?: boolean
  airy?: boolean
}) {
  return (
    <div className={cx("stack", tight && "tight", airy && "airy", className)} style={style}>
      {children}
    </div>
  )
}

export function Section({ label, children, className }: {
  label: string
  children?: ReactNode
  className?: string
}) {
  return (
    <section className={cx("box", "section", className)}>
      <h2 className="title">{label}</h2>
      {children}
    </section>
  )
}

export function Row({ children, className, style }: {
  children?: ReactNode
  className?: string
  style?: CSSProperties
}) {
  return (
    <div className={cx("row", className)} style={style}>
      {children}
    </div>
  )
}

export function Cluster({ children, className, style }: {
  children?: ReactNode
  className?: string
  style?: CSSProperties
}) {
  return (
    <div className={cx("cluster", className)} style={style}>
      {children}
    </div>
  )
}

// GRID

export function Grid({ children, className, style, cols, min }: {
  children?: ReactNode
  className?: string
  style?: CSSProperties
  cols?: number
  min?: number
}) {
  const vars: Record<string, string | number> = {}
  if (cols) vars["--grid-cols"] = cols
  if (min) vars["--min-col-width"] = `${min}px`
  return (
    <div className={cx("grid", className)} style={{ ...style, ...vars } as CSSProperties}>
      {children}
    </div>
  )
}

// CARD

export function Card({ children, className, style, active, onClick, plate }: {
  children?: ReactNode
  className?: string
  style?: CSSProperties
  active?: boolean
  onClick?: () => void
  plate?: ColorName | "auto"
}) {
  const color = usePlate(plate)
  const skin = cx("card", active && "active", className)
  const paint = plated(style, color)
  if (onClick) {
    return (
      <button type="button" className={skin} style={paint} onClick={onClick}>
        {children}
      </button>
    )
  }
  return (
    <div className={skin} style={paint}>
      {children}
    </div>
  )
}
