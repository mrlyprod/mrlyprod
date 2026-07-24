export type Args = Record<string, unknown>

export type Call = { verb: string; args: Args; now?: number }

export type Effect = { kind: string; data: unknown; call?: Call }

export type Verb = { verb: string; args: Args }

export type Route = { app: string; view: string; params: Args }

export type Outcome = { ok: boolean; data: unknown; note: string | null }

export type Notice = { title: string; body: string; at: number }

export type Sync = "pending" | "synced" | "failed"

export type Manifest = {
  route: string
  emoji: string
  title: string
  category: string
  hidden: boolean
  internet: boolean
}

export type Sym = { as: "emoji" | "icon" | "glyph"; value: string }

export type Raster = { text: string; width: number; height: number; rows: number[][] }

export type Shade = { program: string; uniforms: number[]; route?: string; mesh?: string }

export type Flip = { rows: number[][]; palette: string[] }

export type Shaders = Record<string, string>

export type Palette = { names: string[]; hex: Record<string, string>; canvas: { dark: string; light: string } }

export type Mark = { rows: number; cols: number; fps: number; frames: number[][] }

export type Node =
  | { kind: "Stack"; key?: string; children: Node[] }
  | { kind: "Grid"; key?: string; cols: number; mode?: "snap"; children: Node[] }
  | { kind: "Card"; key?: string; children: Node[] }
  | { kind: "Pills"; key?: string; children: Node[] }
  | { kind: "Text"; key?: string; text: string; role?: "title" | "label" | "note" | "body"; fx?: "scramble" }
  | { kind: "Symbol"; key?: string; as: Sym["as"]; value: string }
  | { kind: "Label"; key?: string; symbol?: Sym; text?: string; note?: string; mode: "row" | "stack" | "icon" | "text"; call?: Call; href?: string; fx?: "scramble" }
  | { kind: "Image"; key?: string; src: string; alt: string }
  | { kind: "Canvas"; key?: string; handle: string; rows: number[][]; palette?: string[]; shade?: Shade; strip?: Flip[]; tap?: Call; drag?: Call; turn?: Call; zoom?: Call; pan?: Call; grid?: [number, number] }
  | { kind: "Button"; key?: string; label: string; call?: Call; active?: boolean; bg?: string; big?: boolean; press?: Call; lift?: Call }
  | { kind: "Field"; key?: string; value: string; live: boolean; call: Call; arg: string; label?: string; hint?: string; icon?: string; clear?: boolean; enter?: Call }
  | { kind: "Toggle"; key?: string; on: boolean; call: Call; arg: string; label?: string }
  | { kind: "Choice"; key?: string; value: string; options: string[]; call: Call; arg: string; label?: string; mode?: "row" | "cycle" | "select" }
  | { kind: "Range"; key?: string; value: number; min: number; max: number; step?: number; call: Call; arg: string; label?: string }
  | { kind: "Overlay"; key?: string; child: Node; close?: Call }
  | { kind: "Cell"; key?: string; child?: Node; call?: Call; on?: boolean; bg?: string }
  | { kind: "Doc"; key?: string; md: string; code?: string; handle?: string; open?: Call }
  | { kind: "Cells"; key?: string; rows: string[][] }
  | { kind: "Mark"; key?: string; doodle?: string }

export type View = {
  app: string
  params: Args
  state: unknown
  actions: Verb[]
  beat?: Call
}

export type Observation = {
  tick: number
  route: Route | null
  view: View | null
  last: Outcome | null
  sync: Sync
  effects?: Effect[]
  notices?: Notice[]
}

export type Send = (call: Call, beat?: boolean) => void

export type Registry = {
  version: string
  apps: Manifest[]
  verbs: { app: string; verbs: Verb[] }[]
  nav: Verb[]
}

export type Held = HTMLElement & { __node?: Node; __committed?: string; __tint?: string }
