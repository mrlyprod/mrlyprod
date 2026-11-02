import type { CSSProperties } from "react";
import type { Props } from "../../net/shell/design.ts";
import { DARK, LIGHT } from "./skin.ts";

/* PREVIEW */

const SKINS = { light: LIGHT, dark: DARK };

const tone = (theme: Props["preview"]["theme"]) =>
  Object.fromEntries(Object.entries(SKINS[theme]).filter(([name]) => name !== "--accent")) as CSSProperties;

export function Preview({ theme }: Props["preview"]) {
  return (
    <span className="classic still" data-tone={theme} style={tone(theme)} aria-hidden="true">
      <span className="still-win">
        <span className="still-title">
          <i />
          <i />
          <i />
        </span>
        <span className="still-body" />
      </span>
      <span className="still-task">
        <i />
      </span>
    </span>
  );
}
