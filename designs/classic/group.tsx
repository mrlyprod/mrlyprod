import { loose } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";
import { Glyph } from "./glyphs.tsx";

/* GROUP */

export function Group({ name, open, live, onToggle, bind, children }: Props["group"]) {
  if (open === undefined)
    return (
      <fieldset className="classic group" {...bind?.()}>
        {name && <legend>{name}</legend>}
        <div className="row">{children}</div>
      </fieldset>
    );
  return (
    <details
      className="classic group fold"
      open={open ?? undefined}
      onToggle={(e) => onToggle?.(e.currentTarget.open)}
      {...bind?.()}
    >
      <summary onClick={loose}>
        <span className="sunken sign">
          <span className="shut">
            <Glyph art="plus" />
          </span>
          <span className="open">
            <Glyph art="minus" />
          </span>
        </span>
        <span className="name">{name}</span>
        <span className="live">{live ?? ""}</span>
      </summary>
      <div className="row">{children}</div>
    </details>
  );
}
