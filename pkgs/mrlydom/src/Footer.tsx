import type { ReactNode } from "react"

export function Footer({ children }: { children?: ReactNode }) {
  return (
    <footer>
      {children}
      <p className="fine">Copyright © MrlyProd, Inc. 2013-2026</p>
    </footer>
  )
}
