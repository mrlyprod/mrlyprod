import type { CSSProperties } from "react";
import type { Props } from "../../net/shell/design.ts";
import { BRICKS } from "./brick.ts";
import { SKIN } from "./skin.ts";

/* PREVIEW */

const tone = (theme: Props["preview"]["theme"]) =>
  Object.fromEntries(Object.entries(SKIN[theme]).filter(([name]) => name !== "--accent")) as CSSProperties;

export function Preview({ theme }: Props["preview"]) {
  return (
    <span className="still" {...BRICKS} data-tone={theme} style={tone(theme)} aria-hidden="true">
      <span className="still-rail">
        <i />
        <i />
        <i />
      </span>
      <span className="still-win">
        <span className="still-head">
          <i />
        </span>
      </span>
    </span>
  );
}
