import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Sprite } from "./sprite.tsx";

const quiet = (e: { detail: number; currentTarget: { blur: () => void } }) => e.detail > 0 && e.currentTarget.blur();

export function Group({ name, open, live, onToggle, bind, children }: Props["group"]) {
  if (open === undefined)
    return (
      <fieldset className="pixel group" {...bind?.()}>
        {name && <legend>{name}</legend>}
        <div className="rows">{children}</div>
      </fieldset>
    );
  return (
    <details
      className="pixel group fold"
      open={open ?? undefined}
      onToggle={(e) => onToggle?.(e.currentTarget.open)}
      {...bind?.()}
    >
      <summary onClick={quiet}>
        <span className="sign" aria-hidden="true">
          <Sprite art="right" size={12} />
        </span>
        <span className="name">{name}</span>
        <span className="live">{live ?? ""}</span>
      </summary>
      <div className="rows">{children}</div>
    </details>
  );
}
