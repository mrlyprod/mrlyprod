import type { Props } from "@mrlyprod/ui/shell/design.ts";
import type { CSSProperties } from "react";
import { SKIN } from "../skin.ts";

const tone = (theme: Props["preview"]["theme"]) =>
  Object.fromEntries(Object.entries(SKIN[theme]).filter(([name]) => name !== "--accent")) as CSSProperties;

export function Preview({ theme }: Props["preview"]) {
  return (
    <span className="still" data-tone={theme} style={tone(theme)} aria-hidden="true">
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
