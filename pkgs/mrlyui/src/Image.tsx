import type { CSSProperties } from "react"
import { cx } from "./lib"

// IMAGE

export function Image({ src, alt, className, style }: {
  src: string
  alt: string
  className?: string
  style?: CSSProperties
}) {
  return <img className={cx("image", className)} style={style} src={src} alt={alt} />
}
