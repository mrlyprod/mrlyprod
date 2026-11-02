import type { Props } from "../../net/shell/design.ts";
import { BRICKS } from "./brick.ts";

/* DESK */

export function Desk({ home }: Props["desk"]) {
  return <div className="desk studs" {...BRICKS} data-home={home} aria-hidden="true" />;
}
