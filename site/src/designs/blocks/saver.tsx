import type { Props } from "../../shell/design.ts";
import { BLOCKS, day, paint, time } from "./brick.ts";

/* SAVER */

export function Saver({ clock: given, stage }: Props["saver"]) {
  const clock = given && { ...given, hours: given.hours ?? 24 };
  return (
    <div className="saver" {...BLOCKS} {...stage()}>
      {clock && (
        <div className="clock brick" style={paint("white")}>
          <time className="time">{time(clock)}</time>
          <time className="date">{day(clock)}</time>
        </div>
      )}
    </div>
  );
}
