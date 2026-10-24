import { Launchpad, opened } from "../../shell/launchpad.jsx";
import { hide, show } from "../../shell/parts.jsx";

export const layout = { wide: true, bare: true, window: "none" };

export function render(_row, _site, host) {
  show(host, <Launchpad />);
}

export const mount = opened;

export const unmount = hide;
