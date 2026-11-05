import type { Site } from "site:routes";
import type { Row } from "../../../kit/rows.ts";
import { Launchpad, opened } from "../../launchpad.tsx";
import { hide, show } from "../../parts.tsx";
import type { Layout } from "../../router.ts";

export const layout: Layout = { wide: true, bare: true, window: "none" };

export function render(_row: Row, _site: Site, host: HTMLElement) {
  show(host, <Launchpad />);
}

export const mount = opened;

export const unmount = hide;
