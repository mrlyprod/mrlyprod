import { Symbol } from "./Glyphs"
import { seti } from "./seti"

// NODES

export type TreeNode = {
  name: string
  href: string
  nodes?: TreeNode[]
}

function holds(node: TreeNode, current: string): boolean {
  if (node.href === current) return true
  return node.nodes !== undefined && node.nodes.some(sub => holds(sub, current))
}

function mark(href: string, current: string): "page" | undefined {
  return href === current ? "page" : undefined
}

// LEVEL

function Level({ nodes, current }: { nodes: TreeNode[]; current: string }) {
  return (
    <>
      {nodes.map(node =>
        node.nodes === undefined ? (
          <a key={node.name} className="leaf" href={node.href} aria-current={mark(node.href, current)}>
            <span className={seti(node.name)} aria-hidden="true" />
            {node.name}
          </a>
        ) : (
          <details key={node.name} open={holds(node, current)}>
            <summary>
              <Symbol name="chevron_right" className="turn" />
              <a href={node.href} aria-current={mark(node.href, current)}>
                {node.name}
              </a>
            </summary>
            <div className="sub">
              <Level nodes={node.nodes} current={current} />
            </div>
          </details>
        ),
      )}
    </>
  )
}

// TREE

export function Tree({ nodes, current = "", root }: {
  nodes: TreeNode[]
  current?: string
  root?: TreeNode
}) {
  return (
    <nav className="tree" aria-label="files">
      {root !== undefined && (
        <a className="root" href={root.href} aria-current={mark(root.href, current)}>
          {root.name}
        </a>
      )}
      <Level nodes={nodes} current={current} />
    </nav>
  )
}
