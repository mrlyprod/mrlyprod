import { own } from "../../lib/app/reader.tsx";
import { figures } from "../../shell/figures.ts";
import type { Mounting } from "../../shell/router.ts";
import { Widget } from "./widget.tsx";

export const { layout, render, mount, unmount } = own<Mounting>((_host, { rows, site }) => (
  <Widget rows={rows} site={site} draw={figures} />
));
