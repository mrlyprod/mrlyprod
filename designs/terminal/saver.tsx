import { stamp } from "../../net/sdk/clock.ts";
import type { Props } from "../../net/shell/design.ts";
import { spot } from "../shared/kit.ts";
import { banner } from "./banner.ts";

/* SAVER */

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
