import { rows, site } from "site:routes";
import { state } from "../kit/history.ts";
import type { Row, Shelf } from "../kit/rows.ts";
import { arrange, docked, recents, visited } from "./launch.ts";
import { KEY, read, write } from "./prefs.ts";
import type { Wired } from "./route.ts";

export const PAD = site.launchpad;
export const KNOWN = new Set(PAD.sections.flatMap((one) => one.apps));

const BY = new Map(rows.flatMap((one): [string, Row][] => (one.meta?.id ? [[one.meta.id, one]] : [])));

export const app = (id: string) => BY.get(id);
export const tell = (name: string) => dispatchEvent(new Event(name));

export const shelves = () => arrange(PAD.sections, read(KEY.launchpad));

export function shelve(next: Shelf[]) {
  write(KEY.launchpad, JSON.stringify(next));
  tell("shelves");
}

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

export type Click = Pick<MouseEvent, "button" | "metaKey" | "ctrlKey" | "shiftKey" | "altKey" | "preventDefault">;

export const plain = (e: Click) => e.button === 0 && !e.metaKey && !e.ctrlKey && !e.shiftKey && !e.altKey;

const up = () => location.pathname === site.menu;

export function seek() {
  if (up()) return tell("find");
  dispatchEvent(new CustomEvent("go", { detail: site.menu }));
}

export function launch(e: Click) {
  if (!plain(e) || !up()) return;
  e.preventDefault();
  away();
}

export function find(e: Click) {
  if (!plain(e) || !up()) return;
  e.preventDefault();
  tell("find");
}
