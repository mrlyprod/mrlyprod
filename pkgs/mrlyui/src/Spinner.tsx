import type { CSSProperties } from "react"

export function Spinner({ size }: { size?: string }) {
  return (
    <span
      className="spinner"
      role="status"
      aria-label="Loading"
      style={size ? ({ "--spinner-size": size } as CSSProperties) : undefined}
    />
  )
}
