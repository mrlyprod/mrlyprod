import { Icon } from "../../net/sdk/icon.tsx";
import { loose } from "../../net/sdk/room.ts";
import type { Props } from "../../net/shell/design.ts";
import { BLOCKS, paint } from "./brick.ts";

/* GROUP */

export function Group({ name, open, live, onToggle, bind, children }: Props["group"]) {
  if (open === undefined)
    return (
      <section className="group plate" {...BLOCKS} aria-label={name || undefined} {...bind?.()}>
        {name && <h3 className="head tile">{name}</h3>}
        <div className="rows">{children}</div>
      </section>
    );
  return (
    <details
      className="group plate"
      {...BLOCKS}
      open={open ?? undefined}
      onToggle={(e) => onToggle?.(e.currentTarget.open)}
      {...bind?.()}
    >
      <summary className="lip studs" onClick={loose}>
        <span className="head tile">
          <h3>{name}</h3>
          <span className="live">{live ?? ""}</span>
        </span>
        <span className="fold brick" style={paint("white")} aria-hidden="true">
          <Icon name="expand_more" size={24} />
        </span>
      </summary>
      <div className="rows">{children}</div>
    </details>
  );
}
