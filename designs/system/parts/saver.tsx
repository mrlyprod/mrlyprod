import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { day, time } from "../own/cell.ts";

export function Saver({ clock, stage }: Props["saver"]) {
  return (
    <div className="saver" {...stage()}>
      {clock && (
        <div className="clock">
          <time className="time cell">{time(clock)}</time>
          <time className="date cell">{day(clock)}</time>
        </div>
      )}
    </div>
  );
}
