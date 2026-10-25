import type { Site } from "site:routes";
import type { Row } from "../../../scripts/build.ts";
import { hide, show } from "../../shell/parts.tsx";
import type { Layout } from "../../shell/router.ts";

export const layout: Layout = { wide: true, bare: true, window: "none" };

export function render(_row: Row, site: Site, host: HTMLElement) {
  show(host, <h1 className="visually-hidden">{site.title}</h1>);
}

export const unmount = hide;
