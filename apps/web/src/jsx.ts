import type { Node } from "./types.ts"

type Kind = Node["kind"]
type ByKind<K extends Kind> = Extract<Node, { kind: K }>
export type Kids = Node | boolean | null | undefined | Kids[]
export type Component<P = Record<string, unknown>> = (props: P) => Node
type Words = string | number | boolean | null | undefined | Words[]
type Bare<K extends Kind> = Omit<ByKind<K>, "kind" | "key"> & { key: string }
type Props<K extends Kind> = K extends "Stack" | "Grid" | "Card" | "Pills"
  ? Omit<Bare<K>, "children"> & { children?: Kids }
  : K extends "Overlay"
    ? Omit<Bare<K>, "child"> & { children: Kids }
    : K extends "Cell"
    ? Omit<Bare<K>, "child"> & { children?: Kids }
    : K extends "Text"
      ? Omit<Bare<K>, "text"> & { children?: Words }
      : K extends "Button"
        ? Omit<Bare<K>, "label"> & { children?: Words }
        : Bare<K>
type Tags = { [K in Kind as Lowercase<K>]: Props<K> }

declare global {
  namespace JSX {
    type Element = Node
    type IntrinsicElements = Tags
    interface ElementChildrenAttribute { children: unknown }
  }
}

const kinds: Record<string, Kind> = {
  stack: "Stack",
  grid: "Grid",
  card: "Card",
  pills: "Pills",
  text: "Text",
  symbol: "Symbol",
  label: "Label",
  image: "Image",
  canvas: "Canvas",
  button: "Button",
  field: "Field",
  toggle: "Toggle",
  choice: "Choice",
  range: "Range",
  overlay: "Overlay",
  cell: "Cell",
  doc: "Doc",
  cells: "Cells",
  mark: "Mark",
}

function clean(props: Record<string, unknown> | null): Record<string, unknown> {
  const out: Record<string, unknown> = {}
  for (const [name, value] of Object.entries(props ?? {})) {
    if (name !== "children" && value !== undefined) out[name] = value
  }
  return out
}

function flat(kids: unknown[]): unknown[] {
  return kids.flat(Infinity).filter(kid => kid !== null && kid !== undefined && typeof kid !== "boolean")
}

function nodes(kids: unknown[]): Node[] {
  return flat(kids).map(kid => {
    if (typeof kid === "object" && "kind" in (kid as object)) return kid as Node
    throw new Error(`bare child ${String(kid)} needs a tag`)
  })
}

function words(kids: unknown[]): string {
  return flat(kids).map(String).join("")
}

export function h<P>(tag: Component<P>, props: Record<string, unknown> | null, ...kids: unknown[]): Node
export function h(tag: string, props: Record<string, unknown> | null, ...kids: unknown[]): Node
export function h(tag: unknown, props: Record<string, unknown> | null, ...kids: unknown[]): Node {
  if (typeof tag === "function") return (tag as Component)({ ...clean(props), children: nodes(kids) })
  const kind = kinds[String(tag)]
  if (kind === undefined) throw new Error(`unknown tag ${String(tag)}`)
  const base = { kind, ...clean(props) }
  if (kind === "Stack" || kind === "Grid" || kind === "Card" || kind === "Pills") return { ...base, children: nodes(kids) } as Node
  if (kind === "Overlay") {
    const inner = nodes(kids)
    if (inner.length !== 1) throw new Error(`overlay needs one child, got ${inner.length}`)
    return { ...base, child: inner[0] as Node } as Node
  }
  if (kind === "Cell") {
    const inner = nodes(kids)
    if (inner.length > 1) throw new Error(`cell takes at most one child, got ${inner.length}`)
    return (inner.length === 1 ? { ...base, child: inner[0] as Node } : base) as Node
  }
  if (kind === "Text") return { ...base, text: words(kids) } as Node
  if (kind === "Button") return { ...base, label: words(kids) } as Node
  if (flat(kids).length > 0) throw new Error(`${String(tag)} takes no children`)
  return base as Node
}
