/* STORAGE */

export const PREFIX = (typeof document !== "undefined" && document.documentElement.dataset.prefix) || "mrly-";
export const KEY = {
  theme: `${PREFIX}theme`,
  font: `${PREFIX}font`,
  tint: `${PREFIX}tint`,
  glass: `${PREFIX}glass`,
  saver: `${PREFIX}saver`,
  cart: `${PREFIX}cart`,
  welcome: `${PREFIX}welcome`,
  page: `${PREFIX}page`,
  wide: `${PREFIX}wide`,
  launchpad: `${PREFIX}launchpad`,
  recent: `${PREFIX}recent`,
  dock: `${PREFIX}dock`,
  widgets: `${PREFIX}widgets`,
  magnify: `${PREFIX}magnify`,
  icons: `${PREFIX}icons`,
  always: `${PREFIX}icons-always`,
  off: `${PREFIX}savers-off`,
  idle: `${PREFIX}idle`,
  clock: `${PREFIX}clock`,
  hours: `${PREFIX}hours`,
  text: `${PREFIX}text`,
  motion: `${PREFIX}motion`,
  clear: `${PREFIX}clear`,
  contrast: `${PREFIX}contrast`,
  targets: `${PREFIX}targets`,
};

export type Name = keyof typeof KEY;

export type Fan = (key: string | null) => void;

export type Access = "text" | "motion" | "contrast" | "targets";

export type Settings = Partial<Record<Name, string>>;

export const read = (key: string) => {
  try {
    return localStorage.getItem(key) ?? "";
  } catch {
    return "";
  }
};

const fans = new Set<Fan>();

function tell(key: string | null) {
  for (const fn of [...fans]) {
    try {
      fn(key);
    } catch (error) {
      console.error(error);
    }
  }
}

export const write = (key: string, value: string) => {
  const was = read(key);
  try {
    if (value) localStorage.setItem(key, value);
    else localStorage.removeItem(key);
  } catch {}
  if (read(key) !== was) tell(key);
};

export function subscribe(fn: Fan) {
  fans.add(fn);
  return () => {
    fans.delete(fn);
  };
}

export const root = () => document.documentElement;

/* THEME */

let painted: string | null = null;

function repaint() {
  const now = `${root().dataset.theme ?? ""} ${root().dataset.tint ?? ""}`;
  if (painted === null) painted = now;
  if (now === painted) return;
  painted = now;
  window.dispatchEvent(new Event("theme"));
}

export function theme(next: string) {
  if (next) root().dataset.theme = next;
  else delete root().dataset.theme;
  write(KEY.theme, next);
  for (const label of document.querySelectorAll("[data-theme-toggle] b")) label.textContent = next || "auto";
  shade();
  repaint();
}

/* TINT */

export function tint(next: string) {
  if (next) root().dataset.tint = next;
  else delete root().dataset.tint;
  write(KEY.tint, next);
  repaint();
}

export function turn() {
  const now = root().dataset.theme ?? "";
  theme(now === "" ? "light" : now === "light" ? "dark" : "");
}

/* GLASS */

const CLEAR = "(prefers-reduced-transparency: reduce)";

export function glass(next: string) {
  const want =
    read(KEY.clear) === "1" || (typeof matchMedia === "function" && matchMedia(CLEAR).matches) ? "off" : next;
  if (want) root().dataset.glass = want;
  else delete root().dataset.glass;
  write(KEY.glass, next);
}

export function clear(on: boolean) {
  write(KEY.clear, on ? "1" : "");
  glass(read(KEY.glass));
}

/* ICONS */

const DARK = "(prefers-color-scheme: dark)";

export function look(style: string, always: boolean, dark: boolean) {
  if (!style) return "";
  const tone = always || dark ? "dark" : "light";
  if (style === "dark") return tone === "dark" ? "dark" : "";
  return `${style}-${tone}`;
}

export function shade() {
  const set = root().dataset.theme;
  const dark = set ? set === "dark" : typeof matchMedia === "function" && matchMedia(DARK).matches;
  const next = look(read(KEY.icons), read(KEY.always) === "1", dark);
  if (next) root().dataset.icons = next;
  else delete root().dataset.icons;
}

export function icons(style: string, always: boolean) {
  write(KEY.icons, style);
  write(KEY.always, always ? "1" : "");
  shade();
}

/* DOCK */

export function magnify(on: boolean) {
  if (on) root().dataset.magnify = "";
  else delete root().dataset.magnify;
  write(KEY.magnify, on ? "1" : "");
}

/* FONT */

export function face(next: string) {
  if (next) root().dataset.font = next;
  else delete root().dataset.font;
  write(KEY.font, next);
}

/* ACCESSIBILITY */

function flag(name: Access, next: string) {
  if (next) root().dataset[name] = next;
  else delete root().dataset[name];
  write(KEY[name], next);
}

export const text = (next: string) => flag("text", next);
export const motion = (next: string) => flag("motion", next);
export const contrast = (next: string) => flag("contrast", next);
export const targets = (next: string) => flag("targets", next);

const CALM = "(prefers-reduced-motion: reduce)";

export const calm = () =>
  (typeof document !== "undefined" && root().dataset.motion === "reduce") ||
  (typeof matchMedia === "function" && matchMedia(CALM).matches);

/* SAVER */

export const screen = (next: string) => write(KEY.saver, next);

export function offs(): string[] {
  try {
    const list: unknown = JSON.parse(read(KEY.off) || "[]");
    return Array.isArray(list) ? list.filter((one): one is string => typeof one === "string") : [];
  } catch {
    return [];
  }
}

export const savers = (list: string[] | null | undefined) => write(KEY.off, list?.length ? JSON.stringify(list) : "");

export const idle = (next: string) => write(KEY.idle, next);

export const clock = (next: string) => write(KEY.clock, next);

export function minutes() {
  const next = read(KEY.idle);
  if (next === "never") return 0;
  return Number(next) > 0 ? Number(next) : 5;
}

/* CLOCK */

const CYCLES: Partial<Record<string, "h12" | "h23">> = { 12: "h12", 24: "h23" };

export const hours = (next: string) => write(KEY.hours, next);

export function stamp(now: Date, mode: string, bare?: boolean) {
  const cycle = CYCLES[mode];
  return new Intl.DateTimeFormat(undefined, { hour: "numeric", minute: "2-digit", ...(cycle && { hourCycle: cycle }) })
    .formatToParts(now)
    .filter((part) => !bare || part.type !== "dayPeriod")
    .map((part) => part.value)
    .join("")
    .trim();
}

/* REPLAY */

function width(name: "page" | "wide", prop: string) {
  const px = Number(read(KEY[name]));
  if (px > 0) root().style.setProperty(prop, `${px}px`);
  else root().style.removeProperty(prop);
}

const REPLAY: Partial<Record<Name, () => void>> = {
  theme: () => theme(read(KEY.theme)),
  tint: () => tint(read(KEY.tint)),
  font: () => face(read(KEY.font)),
  glass: () => glass(read(KEY.glass)),
  clear: () => glass(read(KEY.glass)),
  icons: shade,
  always: shade,
  magnify: () => magnify(read(KEY.magnify) === "1"),
  text: () => flag("text", read(KEY.text)),
  motion: () => flag("motion", read(KEY.motion)),
  contrast: () => flag("contrast", read(KEY.contrast)),
  targets: () => flag("targets", read(KEY.targets)),
  saver: () => screen(read(KEY.saver)),
  page: () => width("page", "--win"),
  wide: () => width("wide", "--wide"),
};

export function replay(key: string | null) {
  for (const [name, fn] of Object.entries(REPLAY) as [Name, () => void][]) {
    if (key !== null && key !== KEY[name]) continue;
    try {
      fn();
    } catch (error) {
      console.error(error);
    }
  }
}

if (typeof window !== "undefined") {
  window.addEventListener("storage", (e) => {
    replay(e.key);
    tell(e.key);
  });
}

/* SETTINGS */

export const SETTINGS =
  "theme tint font text glass icons always magnify saver off idle clock hours motion clear contrast targets dock widgets launchpad page wide".split(
    " ",
  ) as Name[];

export const pack = (): Settings =>
  Object.fromEntries(SETTINGS.map((name) => [name, read(KEY[name])]).filter(([, value]) => value));

export function unpack(data: unknown) {
  const kind = data && typeof data === "object" ? Object.getPrototypeOf(data) : undefined;
  if (kind !== Object.prototype && kind !== null) throw new Error("This is not a settings file.");
  for (const [name, value] of Object.entries(data as Record<string, unknown>)) {
    if (!(SETTINGS as string[]).includes(name)) throw new Error(`Unknown setting "${name}".`);
    if (typeof value !== "string") throw new Error(`The setting "${name}" is not text.`);
  }
  const given = data as Record<string, string>;
  for (const name of SETTINGS) write(KEY[name], Object.hasOwn(given, name) ? given[name] : "");
}

export function encode(data: unknown) {
  const bytes = new TextEncoder().encode(JSON.stringify(data));
  return btoa(Array.from(bytes, (byte) => String.fromCharCode(byte)).join(""))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");
}

export function decode(code: unknown): Record<string, unknown> | null {
  if (typeof code !== "string" || !/^[\w-]+$/.test(code)) return null;
  try {
    const bin = atob(
      code
        .replace(/-/g, "+")
        .replace(/_/g, "/")
        .padEnd(Math.ceil(code.length / 4) * 4, "="),
    );
    const data: unknown = JSON.parse(
      new TextDecoder("utf-8", { fatal: true }).decode(Uint8Array.from(bin, (char) => char.charCodeAt(0))),
    );
    return data && typeof data === "object" && !Array.isArray(data) ? (data as Record<string, unknown>) : null;
  } catch {
    return null;
  }
}

export const share = (origin: string) => `${origin}/settings/?apply=${encode(pack())}`;

export function stored(): [string, string][] {
  const rows: [string, string][] = [];
  try {
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key?.startsWith(PREFIX)) rows.push([key, localStorage.getItem(key) ?? ""]);
    }
  } catch {}
  return rows.sort(([a], [b]) => (a < b ? -1 : 1));
}
