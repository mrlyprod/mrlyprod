import { loose } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";
import { BOXY } from "./cell.ts";

/* GROUP */

export function Group({ name, open, live, onToggle, bind, children }: Props["group"]) {
  if (open === undefined)
    return (
      <section className="group" {...BOXY} aria-label={name || undefined} {...bind?.()}>
        {name && <h3 className="head cell">{name}</h3>}
        <div className="rows">{children}</div>
      </section>
    );
  return (
    <details
      className="group"
      {...BOXY}
      open={open ?? undefined}
      onToggle={(e) => onToggle?.(e.currentTarget.open)}
      {...bind?.()}
    >
      <summary className="head cell" onClick={loose}>
        <span className="fold" aria-hidden="true" />
        <h3>{name}</h3>
        <span className="live">{live ?? ""}</span>
      </summary>
      <div className="rows">{children}</div>
    </details>
  );
}
