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

export const read = (key) => {
  try {
    return localStorage.getItem(key) ?? "";
  } catch {
    return "";
  }
};

const fans = new Set();

function tell(key) {
  for (const fn of [...fans]) {
    try {
      fn(key);
    } catch (error) {
      console.error(error);
    }
  }
}

export const write = (key, value) => {
  const was = read(key);
  try {
    if (value) localStorage.setItem(key, value);
    else localStorage.removeItem(key);
  } catch {}
  if (read(key) !== was) tell(key);
};

export function subscribe(fn) {
  fans.add(fn);
  return () => {
    fans.delete(fn);
  };
}

export const root = () => document.documentElement;

/* THEME */

let painted = null;

function repaint() {
  const now = `${root().dataset.theme ?? ""} ${root().dataset.tint ?? ""}`;
  if (painted === null) painted = now;
  if (now === painted) return;
  painted = now;
  window.dispatchEvent(new Event("theme"));
}

export function theme(next) {
  if (next) root().dataset.theme = next;
  else delete root().dataset.theme;
  write(KEY.theme, next);
  for (const label of document.querySelectorAll("[data-theme-toggle] b")) label.textContent = next || "auto";
  shade();
  repaint();
}

/* TINT */

export function tint(next) {
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

export function glass(next) {
  const want =
    read(KEY.clear) === "1" || (typeof matchMedia === "function" && matchMedia(CLEAR).matches) ? "off" : next;
  if (want) root().dataset.glass = want;
  else delete root().dataset.glass;
  write(KEY.glass, next);
}

export function clear(on) {
  write(KEY.clear, on ? "1" : "");
  glass(read(KEY.glass));
}

/* ICONS */

const DARK = "(prefers-color-scheme: dark)";

export function look(style, always, dark) {
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

export function icons(style, always) {
  write(KEY.icons, style);
  write(KEY.always, always ? "1" : "");
  shade();
}

/* DOCK */

export function magnify(on) {
  if (on) root().dataset.magnify = "";
  else delete root().dataset.magnify;
  write(KEY.magnify, on ? "1" : "");
}

/* FONT */

export function face(next) {
  if (next) root().dataset.font = next;
  else delete root().dataset.font;
  write(KEY.font, next);
}

/* ACCESSIBILITY */

function flag(name, next) {
  if (next) root().dataset[name] = next;
  else delete root().dataset[name];
  write(KEY[name], next);
}

export const text = (next) => flag("text", next);
export const motion = (next) => flag("motion", next);
export const contrast = (next) => flag("contrast", next);
export const targets = (next) => flag("targets", next);

const CALM = "(prefers-reduced-motion: reduce)";

export const calm = () =>
  (typeof document !== "undefined" && root().dataset.motion === "reduce") ||
  (typeof matchMedia === "function" && matchMedia(CALM).matches);

/* SAVER */

export const screen = (next) => write(KEY.saver, next);

export function offs() {
  try {
    const list = JSON.parse(read(KEY.off) || "[]");
    return Array.isArray(list) ? list.filter((one) => typeof one === "string") : [];
  } catch {
    return [];
  }
}

export const savers = (list) => write(KEY.off, list?.length ? JSON.stringify(list) : "");

export const idle = (next) => write(KEY.idle, next);

export const clock = (next) => write(KEY.clock, next);

export function minutes() {
  const next = read(KEY.idle);
  if (next === "never") return 0;
  return Number(next) > 0 ? Number(next) : 5;
}

/* CLOCK */

const CYCLES = { 12: "h12", 24: "h23" };

export const hours = (next) => write(KEY.hours, next);

export function stamp(now, mode, bare) {
  const cycle = CYCLES[mode];
  return new Intl.DateTimeFormat(undefined, { hour: "numeric", minute: "2-digit", ...(cycle && { hourCycle: cycle }) })
    .formatToParts(now)
    .filter((part) => !bare || part.type !== "dayPeriod")
    .map((part) => part.value)
    .join("")
    .trim();
}

/* REPLAY */

function width(name, prop) {
  const px = Number(read(KEY[name]));
  if (px > 0) root().style.setProperty(prop, `${px}px`);
  else root().style.removeProperty(prop);
}

const REPLAY = {
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

export function replay(key) {
  for (const [name, fn] of Object.entries(REPLAY)) {
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
  );

export const pack = () =>
  Object.fromEntries(SETTINGS.map((name) => [name, read(KEY[name])]).filter(([, value]) => value));

export function unpack(data) {
  const kind = data && typeof data === "object" ? Object.getPrototypeOf(data) : undefined;
  if (kind !== Object.prototype && kind !== null) throw new Error("This is not a settings file.");
  for (const [name, value] of Object.entries(data)) {
    if (!SETTINGS.includes(name)) throw new Error(`Unknown setting "${name}".`);
    if (typeof value !== "string") throw new Error(`The setting "${name}" is not text.`);
  }
  for (const name of SETTINGS) write(KEY[name], Object.hasOwn(data, name) ? data[name] : "");
}

export function encode(data) {
  const bytes = new TextEncoder().encode(JSON.stringify(data));
  return btoa(Array.from(bytes, (byte) => String.fromCharCode(byte)).join(""))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");
}

export function decode(code) {
  if (typeof code !== "string" || !/^[\w-]+$/.test(code)) return null;
  try {
    const bin = atob(
      code
        .replace(/-/g, "+")
        .replace(/_/g, "/")
        .padEnd(Math.ceil(code.length / 4) * 4, "="),
    );
    const data = JSON.parse(
      new TextDecoder("utf-8", { fatal: true }).decode(Uint8Array.from(bin, (char) => char.charCodeAt(0))),
    );
    return data && typeof data === "object" && !Array.isArray(data) ? data : null;
  } catch {
    return null;
  }
}

export const share = (origin) => `${origin}/settings/?apply=${encode(pack())}`;

export function stored() {
  const rows = [];
  try {
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key?.startsWith(PREFIX)) rows.push([key, localStorage.getItem(key) ?? ""]);
    }
  } catch {}
  return rows.sort(([a], [b]) => (a < b ? -1 : 1));
}
