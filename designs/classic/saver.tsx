import { stamp } from "@mrlyprod/ui/kit/clock.ts";
import { spot } from "@mrlyprod/ui/shared/kit.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Saver({ clock, stage }: Props["saver"]) {
  return (
    <div className="classic saver">
      <div className="stage" {...stage()} />
      {clock && (
        <time className="big" style={spot(clock.now)}>
          {stamp(clock.now, String(clock.hours ?? ""))}
        </time>
      )}
    </div>
  );
}
