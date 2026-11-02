import { stamp } from "../../lib/clock.ts";
import type { Props } from "../../shell/design.ts";

/* SAVER */

const two = (n: number) => String(n).padStart(2, "0");

const DATE = { weekday: "long", day: "numeric", month: "long" } as const;

export function Saver({ clock, state, stage }: Props["saver"]) {
  return (
    <div className={state ? `saver ${state}` : "saver"} aria-hidden="true" {...stage()}>
      {clock && (
        <div className="clock">
          <p>{clock.now.toLocaleDateString(undefined, DATE)}</p>
          <time dateTime={`${two(clock.now.getHours())}:${two(clock.now.getMinutes())}`}>
            {stamp(clock.now, clock.hours ? String(clock.hours) : "", true)}
          </time>
        </div>
      )}
    </div>
  );
}
