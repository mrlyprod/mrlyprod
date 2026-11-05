import { stamp } from "@mrlyprod/ui/kit/clock.ts";
import { spot } from "@mrlyprod/ui/shared/kit.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { banner } from "./banner.ts";

export function Saver({ clock, stage }: Props["saver"]) {
  const time = clock && stamp(clock.now, String(clock.hours ?? ""));
  return (
    <div className="terminal saver">
      <div className="stage" {...stage()} />
      {clock && time && (
        <time className="big" style={spot(clock.now)} dateTime={time}>
          <pre role="img" aria-label={time}>
            {banner(time)}
          </pre>
        </time>
      )}
    </div>
  );
}
