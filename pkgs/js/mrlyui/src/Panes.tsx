import type { ReactNode } from "react"
import { Box } from "./Box"
import { Drawer } from "./Drawer"
import { Grip } from "./Grip"
import type { PaneSet } from "./pane"

export function Panes({ panes, left, right, leftTitle = "menu", rightTitle = "iden", children }: {
  panes: PaneSet
  left?: ReactNode
  right?: ReactNode
  leftTitle?: string
  rightTitle?: string
  children?: ReactNode
}) {
  if (!panes.wide) {
    return (
      <>
        <Drawer open={panes.left.open} onClose={panes.left.toggle} side="left" title={leftTitle}>
          {left}
        </Drawer>
        <Drawer open={panes.right.open} onClose={panes.right.toggle} side="right" title={rightTitle}>
          {right}
        </Drawer>
        <div className="main">{children}</div>
      </>
    )
  }
  return (
    <div className="panes">
      {panes.left.open && (
        <>
          <Box className="pane left" level={2} ref={panes.left.hold}>
            {left}
          </Box>
          <Grip side="left" pane={panes.left} />
        </>
      )}
      <Box className="main" level={2}>{children}</Box>
      {panes.right.open && (
        <>
          <Grip side="right" pane={panes.right} />
          <Box className="pane right" level={2} ref={panes.right.hold}>
            {right}
          </Box>
        </>
      )}
    </div>
  )
}

export function Chrome({ children, className }: { children?: ReactNode; className?: string }) {
  return <div className={className ? `chrome ${className}` : "chrome"}>{children}</div>
}
