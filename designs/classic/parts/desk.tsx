import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Desk({ canvas, home }: Props["desk"]) {
  return (
    <div className="wallpaper" data-home={home || undefined}>
      <canvas className="ground" {...canvas()} />
    </div>
  );
}
