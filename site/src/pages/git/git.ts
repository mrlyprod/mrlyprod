import { start } from "../../lib/git/client.ts";
import type { View } from "../../lib/git/view.ts";
import type { Link } from "../../lib/md/md.ts";

/* MARKDOWN */

async function md(text: string, link: Link) {
  const [{ front, render }, katex] = await Promise.all([
    import("../../lib/md/md.ts"),
    text.includes("$") ? import("katex") : null,
  ]);
  const math = katex
    ? (tex: string, display: boolean) =>
        katex.default.renderToString(tex, { output: "mathml", throwOnError: false, displayMode: display })
    : undefined;
  return render(front(text).body, { math, link });
}

const paint = () => import("../../lib/git/code.ts").then((one) => one.paint);

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
