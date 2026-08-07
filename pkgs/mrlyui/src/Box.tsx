import { createContext, useContext, useMemo } from "react"
import type { CSSProperties, ReactNode, Ref } from "react"
import { cx } from "./lib"
import { css, randomColor } from "./colors"
import type { ColorName } from "./colors"
import { useFills } from "./prefs"

// LEVEL

export type Level = 0 | 1 | 2

const Altitude = createContext<Level>(2)

export function useLevel(): Level {
  return useContext(Altitude)
}

function below(level: Level): Level {
  return level > 0 ? ((level - 1) as Level) : 0
}

// PLATE

export function usePlate(plate: ColorName | "auto" | undefined, level: Level): string | undefined {
  const fills = useFills()
  return useMemo(() => {
    if (plate) return css(plate === "auto" ? randomColor() : plate)
    const fill = fills[level]
    if (fill === "random") return css(randomColor())
    if (fill !== undefined && fill !== "") return css(fill)
    return undefined
  }, [plate, fills, level])
}

export function plated(style: CSSProperties | undefined, color: string | undefined): CSSProperties | undefined {
  if (!color) return style
  return { ...style, "--plate": color } as CSSProperties
}

// BOX

export function Box({ children, className, style, onClick, plate, level, ref }: {
  children?: ReactNode
  className?: string
  style?: CSSProperties
  onClick?: () => void
  plate?: ColorName | "auto"
  level?: Level
  ref?: Ref<HTMLDivElement>
}) {
  const held = useLevel()
  const at = level ?? held
  const color = usePlate(plate, at)
  return (
    <Altitude.Provider value={below(at)}>
      <div className={cx("box", `l${at}`, className)} style={plated(style, color)} onClick={onClick} ref={ref}>
        {children}
      </div>
    </Altitude.Provider>
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

export function Section({ label, children, className, plate }: {
  label: string
  children?: ReactNode
  className?: string
  plate?: ColorName | "auto"
}) {
  const color = usePlate(plate, 2)
  return (
    <Altitude.Provider value={1}>
      <section className={cx("box", "l2", "section", className)} style={plated(undefined, color)}>
        <h2 className="title">{label}</h2>
        {children}
      </section>
    </Altitude.Provider>
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
  else if (cols) vars["--min-col-width"] = "0px"
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
  const held = useLevel()
  const color = usePlate(plate, held)
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
