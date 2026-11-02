import type { Props } from "../../shell/design.ts";

/* DESK */

export function Desk({ canvas }: Props["desk"]) {
  return <canvas {...canvas()} />;
}
