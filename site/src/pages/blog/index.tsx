import type { Row } from "../../../scripts/build.ts";
import { hub } from "../../shell/hub.tsx";
import { Grid } from "../../shell/parts.tsx";

const posts = (rows: Row[]) =>
  rows
    .filter((one) => one.meta?.prose === "post")
    .map((one) => ({ name: one.title, href: one.route, figure: one.figure, text: one.lead, dates: [one.date!] }));

export const { layout, render, unmount } = hub((_row, _site, { rows }) => <Grid nodes={posts(rows)} />, { bare: true });
