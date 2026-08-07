export type Args = Record<string, unknown>

export type Call = { verb: string; args: Args; now?: number }

export type Effect = { kind: string; data: unknown; call?: Call }

export type Verb = { verb: string; args: Args }

export type Route = { app: string; view: string; params: Args }

export type Outcome = { ok: boolean; data: unknown; note: string | null }

export type Notice = { title: string; body: string; at: number }

export type Sync = "pending" | "synced" | "failed"

export type KeySet = Partial<Record<"up" | "down" | "left" | "right", Call>>

export type Manifest = {
  route: string
  emoji: string
  title: string
  category: string
  hidden: boolean
  internet: boolean
  keys?: KeySet
}

export type Sym = { as: "emoji" | "icon" | "glyph"; value: string }

export type Raster = { text: string; width: number; height: number; rows: number[][] }

export type Shade = { program: string; route?: string; mesh?: string }

export type Flip = { rows: number[][]; palette: string[] }

export type Pen = { pen: number }

export type Visual = { bg?: string | Pen; motif?: string; face?: { as: "glyph" | "emoji" | "sprite"; value?: string; rows?: number[][]; tint?: "ink" | Pen } }

export type Skins = Record<string, Record<string, Visual[]>>

export type Cells = { ids: number[][]; skin: string; pens: string[]; design?: string }

export type Shaders = Record<string, string>

export type Palette = { names: string[]; hex: Record<string, string>; canvas: { dark: string; light: string } }

export type Mark = { rows: number; cols: number; fps: number; frames: number[][] }

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

export type Slot = { state: unknown }

export type Registry = {
  version: string
  apps: Manifest[]
  verbs: { app: string; verbs: Verb[] }[]
  nav: Verb[]
}
