import type { Props } from "../../net/shell/design.ts";
import { banner } from "./banner.ts";
import { hhmm, spot } from "./clock.ts";

/* SAVER */

export function Saver({ clock, stage }: Props["saver"]) {
  return (
    <div className="terminal saver">
      <div className="stage" {...stage()} />
      {clock && (
        <time className="big" style={spot(clock.now)} dateTime={hhmm(clock)}>
          <pre role="img" aria-label={hhmm(clock)}>
            {banner(hhmm(clock))}
          </pre>
        </time>
      )}
    </div>
  );
}
