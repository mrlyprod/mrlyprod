import type { Props } from "../../net/shell/design.ts";

/* DESK */

export function Desk({ canvas }: Props["desk"]) {
  return <canvas {...canvas()} />;
}
