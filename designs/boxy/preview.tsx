import type { CSSProperties } from "react";
import type { Props } from "../../net/shell/design.ts";
import { BOXY } from "./cell.ts";
import { SKIN } from "./skin.ts";

/* PREVIEW */

const tone = (theme: Props["preview"]["theme"]) =>
  Object.fromEntries(Object.entries(SKIN[theme]).filter(([name]) => name !== "--accent")) as CSSProperties;

export function Preview({ theme }: Props["preview"]) {
  return (
    <span className="still" {...BOXY} data-tone={theme} style={tone(theme)} aria-hidden="true">
      <span className="still-bar">
        <i />
        <i />
        <i />
      </span>
      <span className="still-win">
        <i />
        <i />
        <i />
        <i />
      </span>
    </span>
  );
}
