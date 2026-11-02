import { stamp } from "../../net/sdk/clock.ts";
import type { Props } from "../../net/shell/design.ts";
import { spot } from "../shared/kit.ts";

/* SAVER */

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
