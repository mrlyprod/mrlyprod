import type { Site } from "site:routes";
import type { Row } from "../../../scripts/build.ts";
import { hide, Lost, show } from "../../shell/parts.tsx";
import type { Context, Layout } from "../../shell/router.ts";

export const layout: Layout = { bare: true };

export function render(row: Row, site: Site, host: HTMLElement, { rows }: Context) {
  show(host, <Lost row={row} site={site} rows={rows} />);
}

export const unmount = hide;
