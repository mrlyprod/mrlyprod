import type { Site } from "site:routes";
import type { ReactNode } from "react";
import type { Row } from "../../scripts/build.ts";
import { hide, Lede, show } from "./parts.tsx";
import type { Context, Layout, Own } from "./router.ts";

/* TYPES */

export type Body = (row: Row, site: Site, context: Context) => ReactNode;

export function hub(body: Body, layout: Layout = { wide: true, bare: true }): Own {
  return {
    layout,
    render(row, site, host, context) {
      show(
        host,
        <>
          <Lede id={row.meta.id} title={row.title} lead={row.lead} />
          {body(row, site, context)}
        </>,
      );
    },
    unmount: hide,
  };
}
