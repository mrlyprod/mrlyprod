import { Icon } from "../../lib/app/icon.tsx";
import { loose } from "../../lib/app/room.ts";
import type { Props } from "../../shell/design.ts";

/* GROUP */

export function Group({ name, open, live, onToggle, bind, children }: Props["group"]) {
  if (open === undefined)
    return (
      <section aria-label={name || undefined} {...bind?.()}>
        {name && <h3>{name}</h3>}
        <div className="row">{children}</div>
      </section>
    );
  return (
    <details
      className="group"
      open={open ?? undefined}
      onToggle={(e) => onToggle?.(e.currentTarget.open)}
      {...bind?.()}
    >
      <summary onClick={loose}>
        <h3>{name}</h3>
        <span className="live">{live ?? ""}</span>
        <Icon name="expand_more" />
      </summary>
      <div className="row">{children}</div>
    </details>
  );
}
