import type { Props } from "../../net/shell/design.ts";

/* DESK */

export function Desk({ canvas, home }: Props["desk"]) {
  return (
    <div className="terminal desk" data-home={home || undefined}>
      <canvas className="ground" {...canvas()} />
    </div>
  );
}
