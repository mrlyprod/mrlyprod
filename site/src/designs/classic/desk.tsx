import type { Props } from "../../shell/design.ts";

/* DESK */

export function Desk({ canvas, home }: Props["desk"]) {
  return (
    <div className="classic desk" data-home={home || undefined}>
      <canvas className="ground" {...canvas()} />
    </div>
  );
}
