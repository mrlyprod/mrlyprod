import type { Row } from "@mrlyprod/ui/kit/rows.ts";
import { hide, parts, show } from "../../shell.ts";

const ORDER = ["mrlynet", "mrlymath", "sequences", "crates", "bindgen", "draw", "mrlyprod", "setup"];

const card = (one: Row) => ({ href: one.route, name: one.title, text: one.lead, figure: one.figure ?? undefined });

export const layout = { wide: true, bare: true };

export function render(row: Row, _site: unknown, host: HTMLElement, { rows }: { rows: Row[] }) {
  const cards = ORDER.flatMap((id) => rows.filter((one) => one.meta.id === id)).map(card);
  show(host, <parts.prose kind="hub" id={row.meta.id} title={row.title} lead={row.lead} cards={cards} />);
}

export const unmount = hide;
