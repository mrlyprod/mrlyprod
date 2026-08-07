import type { ReactNode } from "react"
import { cx } from "./lib"
import { Emoji, Symbol } from "./Glyphs"
import type { SymbolName } from "./Glyphs"
import { Letters } from "./Letters"

// SYM

export type Sym =
  | { as: "emoji"; value: string }
  | { as: "icon"; value: SymbolName }
  | { as: "glyph"; value: string }

export type Mode = "row" | "stack" | "icon" | "text"

function face(sym: Sym): ReactNode {
  if (sym.as === "icon") return <Symbol name={sym.value} />
  if (sym.as === "glyph") return <Letters text={sym.value} scramble={false} />
  return <Emoji glyph={sym.value} />
}

// LABEL

export function Label({ symbol, text, note, mode = "row", href, onClick, className }: {
  symbol?: Sym
  text?: string
  note?: string
  mode?: Mode
  href?: string
  onClick?: () => void
  className?: string
}) {
  const sym = mode === "text" ? undefined : symbol
  const words = mode === "icon" ? "" : (text ?? "")
  const gloss = note ?? ""
  const skin = cx("label", mode, className)
  const aria = mode === "icon" ? text : undefined
  const body = (
    <>
      {sym !== undefined && <span className="label-symbol">{face(sym)}</span>}
      {words !== "" && <span className="label-text">{words}</span>}
      {gloss !== "" && <span className="label-note">{gloss}</span>}
    </>
  )
  if (href !== undefined) {
    const saved = href.startsWith("data:")
    return (
      <a
        className={cx(skin, "linked")}
        href={href}
        title={gloss || undefined}
        aria-label={aria}
        download={saved ? (text ?? "download") : undefined}
        target={saved ? undefined : "_blank"}
        rel={saved ? undefined : "noopener"}
      >
        {body}
      </a>
    )
  }
  if (onClick !== undefined) {
    return (
      <button type="button" className={skin} title={gloss || undefined} aria-label={aria} onClick={onClick}>
        {body}
      </button>
    )
  }
  return (
    <span className={skin} title={gloss || undefined} aria-label={aria}>
      {body}
    </span>
  )
}
