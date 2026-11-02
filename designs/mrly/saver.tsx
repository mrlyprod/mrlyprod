import type { Props } from "../../net/shell/design.ts";
import { day, MRLY, time } from "./cell.ts";

/* SAVER */

export function Saver({ clock: given, stage }: Props["saver"]) {
  const clock = given && { ...given, hours: given.hours ?? 24 };
  return (
    <div className="saver" {...MRLY} {...stage()}>
      {clock && (
        <div className="clock">
          <time className="time cell">{time(clock)}</time>
          <time className="date cell">{day(clock)}</time>
        </div>
      )}
    </div>
  );
}
