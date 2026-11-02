import type { Props } from "../../shell/design.ts";
import { BLOCKS } from "./brick.ts";

/* DESK */

export function Desk({ home }: Props["desk"]) {
  return <div className="desk studs" {...BLOCKS} data-home={home} aria-hidden="true" />;
}
