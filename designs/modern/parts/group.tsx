import { Icon } from "@mrlyprod/ui/kit/icon.tsx";
import { loose } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

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
        <Icon name="down" />
      </summary>
      <div className="row">{children}</div>
    </details>
  );
}
