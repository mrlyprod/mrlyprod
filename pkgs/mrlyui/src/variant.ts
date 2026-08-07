import type { SymbolName } from "./Glyphs"

export type Variant = "info" | "success" | "warn" | "danger"

export const VARIANT_ICON: Record<Variant, SymbolName> = {
  info: "info",
  success: "check",
  warn: "info",
  danger: "close",
}
