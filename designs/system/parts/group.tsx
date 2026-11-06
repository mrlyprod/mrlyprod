import { loose } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Group({ name, open, live, onToggle, bind, children }: Props["group"]) {
  if (open === undefined)
    return (
      <section className="group" aria-label={name || undefined} {...bind?.()}>
        {name && <h3 className="head cell">{name}</h3>}
        <div className="rows">{children}</div>
      </section>
    );
  return (
    <details
      className="group"
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
