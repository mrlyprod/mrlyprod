import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Desk({ canvas }: Props["desk"]) {
  return <canvas {...canvas()} />;
}
