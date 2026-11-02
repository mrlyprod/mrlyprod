import type { Props } from "../../net/shell/design.ts";
import { BRICKS, day, paint, time } from "./brick.ts";

/* SAVER */

export function Saver({ clock, stage }: Props["saver"]) {
  return (
    <div className="saver" {...BRICKS} {...stage()}>
      {clock && (
        <div className="clock brick" style={paint("white")}>
          <time className="time">{time(clock)}</time>
          <time className="date">{day(clock)}</time>
        </div>
      )}
    </div>
  );
}
