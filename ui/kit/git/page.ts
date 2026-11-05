import { front, reader } from "../md/md.ts";
import { start } from "./client.ts";
import type { Tools, View } from "./view.ts";

const md: NonNullable<Tools["md"]> = (text, link) => reader().then((one) => one.markdown(front(text).body, { link }));

const paint = () => import("./code.ts").then((one) => one.paint);

export const view = (host: Element, tree: string, after?: (view: View) => void) =>
  start({ tree, mount: host, md, paint, after });

const live = new Map<Element, () => void>();

export function mount(host: Element) {
  if (!live.has(host)) live.set(host, view(host, "/git.json"));
}

export function unmount(host: Element) {
  live.get(host)?.();
  live.delete(host);
}
