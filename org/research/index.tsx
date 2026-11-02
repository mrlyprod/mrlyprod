import { own } from "../../net/sdk/reader.tsx";
import { figures } from "../../net/shell/figures.ts";
import type { Mounting } from "../../net/shell/router.ts";
import { Widget } from "./widget.tsx";

export const { layout, render, mount, unmount } = own<Mounting>((_host, { rows, site }) => (
  <Widget rows={rows} site={site} draw={figures} />
));
