import { loose } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Group({ name, open, live, onToggle, bind, children }: Props["group"]) {
  if (open === undefined)
    return (
      <fieldset className="terminal group" {...bind?.()}>
        {name && <legend>{name}</legend>}
        <div className="row">{children}</div>
      </fieldset>
    );
  return (
    <details
      className="terminal group fold"
      open={open ?? undefined}
      onToggle={(e) => onToggle?.(e.currentTarget.open)}
      {...bind?.()}
    >
      <summary onClick={loose}>
        <span className="sign" aria-hidden="true" />
        <span className="name">{name}</span>
        <span className="live">{live ?? ""}</span>
      </summary>
      <div className="row">{children}</div>
    </details>
  );
}
