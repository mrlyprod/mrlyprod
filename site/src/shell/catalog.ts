import { rows, site } from "site:routes";
import type { Row, Topic } from "../../scripts/build.ts";
import type { Shelf } from "../../scripts/llms.ts";
import { state } from "../lib/app/history.ts";
import { arrange, docked, recents, visited } from "./launch.ts";
import { KEY, read, write } from "./prefs.ts";
import type { Wired } from "./route.ts";

/* TYPES */

export type Note = Topic & { more: string; href: string };

export const PAD = site.launchpad;
export const KNOWN = new Set(PAD.sections.flatMap((one) => one.apps));

const BY = new Map(rows.flatMap((one): [string, Row][] => (one.meta?.id ? [[one.meta.id, one]] : [])));

export const app = (id: string) => BY.get(id);
export const tell = (name: string) => dispatchEvent(new Event(name));

/* SHELVES */

export const shelves = () => arrange(PAD.sections, read(KEY.launchpad));

export function shelve(next: Shelf[]) {
  write(KEY.launchpad, JSON.stringify(next));
  tell("shelves");
}

/* DOCK */

export const dock = () => docked(PAD.dock, read(KEY.dock), KNOWN);

export function redock(next: string[][]) {
  write(KEY.dock, JSON.stringify(next));
  tell("dock");
}

export function unshelve() {
  write(KEY.launchpad, "");
  write(KEY.dock, "");
  tell("shelves");
  tell("dock");
}

/* RECENT */

export const recent = () => recents(read(KEY.recent), KNOWN);

let open = "";
let at = "";

function note() {
  write(
    KEY.recent,
    JSON.stringify(visited(recent(), { id: open, url: location.pathname + location.search, at: Date.now() })),
  );
  tell("recent");
}

export function track() {
  addEventListener("wire", (e: CustomEvent<Wired>) => {
    if (!e.detail) return;
    const one = rows.find((row) => row.route === e.detail?.route);
    const id = one?.meta?.id ?? "";
    open = KNOWN.has(id) ? id : "";
    at = open ? one!.route : "";
    if (open) note();
  });
  addEventListener("stamp", () => open && location.pathname.startsWith(at) && note());
}

/* NOTES */

let topics: Promise<Note[]> | null = null;

export const notes = (): Promise<Note[]> =>
  (topics ??= fetch("/research.json")
    .then((reply) =>
      reply.ok ? reply.json() : Promise.reject(new Error(`research: /research.json answered ${reply.status}`)),
    )
    .then((data: { topics?: Topic[] }) =>
      (data.topics ?? []).map(
        (one): Note => ({
          ...one,
          more: one.lead ?? "",
          href: `/research/${encodeURIComponent(one.slug)}/`,
        }),
      ),
    )
    .catch(() => {
      topics = null;
      return [];
    }));

/* LAUNCH */

let ahead = "";

export const type = (key: string) => {
  ahead += key;
};

export const take = () => {
  const out = ahead;
  ahead = "";
  return out;
};

export const away = () =>
  (state().depth ?? 0) > 0 ? history.back() : dispatchEvent(new CustomEvent("go", { detail: "/" }));
