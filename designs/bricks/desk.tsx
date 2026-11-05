import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { BRICKS } from "./brick.ts";

export function Desk({ home }: Props["desk"]) {
  return <div className="desk studs" {...BRICKS} data-home={home} aria-hidden="true" />;
}
