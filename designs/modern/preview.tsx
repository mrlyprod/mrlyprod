import type { Props } from "../../net/shell/design.ts";

/* PREVIEW */

export function Preview({ theme }: Props["preview"]) {
  return (
    <span className="still" data-tone={theme} aria-hidden="true">
      <span className="still-trio" />
      <span className="still-tray" />
      <span className="still-win">
        <i className="still-close" />
        <i className="still-zoom" />
      </span>
      <span className="still-dock">
        <i />
        <i />
        <i />
        <i />
      </span>
    </span>
  );
}
