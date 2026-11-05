import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Desk({ canvas, home }: Props["desk"]) {
  return (
    <div className="classic desk" data-home={home || undefined}>
      <canvas className="ground" {...canvas()} />
    </div>
  );
}
