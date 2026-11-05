import { start } from "../../kit/git/client.ts";
import type { Tools, View } from "../../kit/git/view.ts";
import { front, reader } from "../../kit/md/md.ts";

const md: NonNullable<Tools["md"]> = (text, link) => reader().then((one) => one.markdown(front(text).body, { link }));

const paint = () => import("./code.ts").then((one) => one.paint);

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
