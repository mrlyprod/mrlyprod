import type { Site } from "site:routes";
import type { Row } from "../../net/sdk/rows.ts";
import { hide, Lede, show } from "../../net/shell/parts.tsx";
import type { Layout } from "../../net/shell/router.ts";

export const layout: Layout = {};

export function render(row: Row, _site: Site, host: HTMLElement) {
  show(
    host,
    <>
      <Lede id="cart" title={row.title} lead={row.lead} />
      <p>mrly.net has no shop yet.</p>
      <p>
        <a href="/">Back to the home page</a>.
      </p>
    </>,
  );
}

export const unmount = hide;
