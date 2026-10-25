import type { Site } from "site:routes";
import type { Row } from "../../../scripts/build.ts";
import { Launchpad, opened } from "../../shell/launchpad.tsx";
import { hide, show } from "../../shell/parts.tsx";
import type { Layout } from "../../shell/router.ts";

export const layout: Layout = { wide: true, bare: true, window: "none" };

export function render(_row: Row, _site: Site, host: HTMLElement) {
  show(host, <Launchpad />);
}

export const mount = opened;

export const unmount = hide;
