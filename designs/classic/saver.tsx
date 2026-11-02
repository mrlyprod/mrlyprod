import type { Props } from "../../net/shell/design.ts";
import { hhmm, spot } from "./clock.ts";

/* SAVER */

export function Saver({ clock, stage }: Props["saver"]) {
  return (
    <div className="classic saver">
      <div className="stage" {...stage()} />
      {clock && (
        <time className="big" style={spot(clock.now)}>
          {hhmm(clock)}
        </time>
      )}
    </div>
  );
}
