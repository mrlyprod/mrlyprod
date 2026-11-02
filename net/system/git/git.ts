import { start } from "../../sdk/git/client.ts";
import type { Tools, View } from "../../sdk/git/view.ts";
import { front, reader } from "../../sdk/md/md.ts";

/* LAZY */

const md: NonNullable<Tools["md"]> = (text, link) => reader().then((one) => one.markdown(front(text).body, { link }));

const paint = () => import("./code.ts").then((one) => one.paint);

/* VIEWER */

export const view = (host: Element, after?: (view: View) => void) =>
  start({ tree: "/git.json", mount: host, md, paint, after });

const live = new Map<Element, () => void>();

export function mount(host: Element) {
  if (!live.has(host)) live.set(host, view(host));
}

export function unmount(host: Element) {
  live.get(host)?.();
  live.delete(host);
}
