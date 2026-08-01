import type { ReactNode } from "react"

type Props = {
  icon: string
  label: string
  open?: boolean
  grid?: boolean
  children: ReactNode
}

export function Fold({ icon, label, open = false, grid = false, children }: Props) {
  return (
    <details className="fold" open={open}>
      <summary>
        <span className="ic" aria-hidden="true">
          {icon}
        </span>
        {label}
        <span className="ic mark" aria-hidden="true">
          chevron_right
        </span>
      </summary>
      <div className={grid ? "body grid" : "body"}>{children}</div>
    </details>
  )
}
