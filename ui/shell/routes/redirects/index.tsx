import type { Site } from "site:routes";
import { Empty, Table } from "../../../kit/controls.tsx";
import type { Row } from "../../../kit/rows.ts";
import { hide, Lede, show } from "../../parts.tsx";
import type { Layout } from "../../router.ts";

export const layout: Layout = {};

export function render(row: Row, site: Site, host: HTMLElement) {
  const moves = Object.entries(site.redirects).sort(
    ([a, x], [b, y]) => y.since.localeCompare(x.since) || a.localeCompare(b),
  );
  show(
    host,
    <>
      <Lede id="redirects" title={row.title} lead={row.lead} />
      {moves.length ? (
        <Table
          head={["From", "To", "Since"]}
          rows={moves.map(([from, one]) => [
            <code key="from">{from}</code>,
            <a key="to" href={one.to}>
              {one.to}
            </a>,
            one.since,
          ])}
        />
      ) : (
        <Empty title="No page has moved" />
      )}
    </>,
  );
}

export const unmount = hide;
