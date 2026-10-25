import type { Shelf } from "../../scripts/llms.ts";

/* TYPES */

export type Seen = { id: string; url: string; at?: unknown };

export type Visit = { id: string; url: string; at: number };

export type Ranked = { title: string; more?: string };

type Loose = { name?: unknown; apps?: unknown } | null;

type Stored = { id?: unknown; url?: unknown; at?: unknown } | null;

export const RECENT = 9;
export const KEPT = 24;
export const SPARE = "New Section";

const two = (n: number) => String(n).padStart(2, "0");
const parse = (text: string): unknown => {
  try {
    return text ? JSON.parse(text) : null;
  } catch {
    return null;
  }
};

/* LAYOUT */

export function arrange(base: Shelf[], text: string): Shelf[] {
  const saved = parse(text);
  const known = new Set(base.flatMap((one) => one.apps));
  const seen = new Set<string>();
  const names = new Set<string>();
  const out: Shelf[] = [];
  for (const one of (Array.isArray(saved) ? saved : []) as Loose[]) {
    if (
      !one ||
      typeof one.name !== "string" ||
      !one.name.trim() ||
      !Array.isArray(one.apps) ||
      names.has(one.name.toLowerCase())
    )
      continue;
    names.add(one.name.toLowerCase());
    out.push({
      name: one.name,
      apps: (one.apps as unknown[]).filter(
        (id): id is string => typeof id === "string" && known.has(id) && !seen.has(id) && Boolean(seen.add(id)),
      ),
    });
  }
  for (const shelf of base) {
    const lost = shelf.apps.filter((id) => !seen.has(id));
    if (!lost.length) continue;
    let home = out.find((one) => one.name.toLowerCase() === shelf.name.toLowerCase());
    if (!home) {
      home = { name: shelf.name, apps: [] };
      out.push(home);
    }
    home.apps.push(...lost);
  }
  return out;
}

export function docked(base: string[][], text: string, known: Set<string>): string[][] {
  const saved = parse(text);
  const seen = new Set<string>();
  const groups = (Array.isArray(saved) && saved.every(Array.isArray) ? (saved as unknown[][]) : base).map((group) =>
    group.filter(
      (id): id is string => typeof id === "string" && known.has(id) && !seen.has(id) && Boolean(seen.add(id)),
    ),
  );
  while (groups.length < base.length) groups.push([]);
  return groups;
}

export function spare(shelves: Shelf[]) {
  const taken = new Set(shelves.map((one) => one.name.toLowerCase()));
  let n = 1;
  while (taken.has((n === 1 ? SPARE : `${SPARE} ${n}`).toLowerCase())) n++;
  return n === 1 ? SPARE : `${SPARE} ${n}`;
}

export function renamed(shelves: Shelf[], from: string, to: string): Shelf[] {
  const name = to.trim();
  if (!name || shelves.some((one) => one.name !== from && one.name.toLowerCase() === name.toLowerCase()))
    return shelves;
  return shelves.map((one) => (one.name === from ? { ...one, name } : one));
}

const line = (ids: string[]) => `[${ids.map((id) => JSON.stringify(id)).join(", ")}]`;

export function exported(shelves: Shelf[], dock: string[][]) {
  const rows = shelves.map((one) => `    { "name": ${JSON.stringify(one.name)}, "apps": ${line(one.apps)} }`);
  return `{\n  "sections": [\n${rows.join(",\n")}\n  ],\n  "dock": [${dock.map(line).join(", ")}]\n}\n`;
}

/* RECENT */

export const siblings = (shelves: Shelf[], id: string) =>
  shelves.find((one) => one.apps.includes(id))?.apps.filter((one) => one !== id) ?? [];

export function visited(list: Seen[], entry: Visit): Seen[] {
  return [entry, ...(Array.isArray(list) ? list : []).filter((one) => one?.id && one.id !== entry.id)].slice(0, KEPT);
}

export function recents(text: string, known: Set<string>): Seen[] {
  const list = parse(text);
  return ((Array.isArray(list) ? list : []) as Stored[]).filter(
    (one): one is Seen =>
      typeof one?.id === "string" && known.has(one.id) && typeof one.url === "string" && one.url.startsWith("/"),
  );
}

export function gist(url: string) {
  const params = new URLSearchParams(url.split("?")[1] ?? "");
  return [...params]
    .filter(([key, value]) => key !== "seed" && value !== "")
    .slice(0, 3)
    .map(([key, value]) => `${key} ${value}`)
    .join(" · ");
}

export function ago(at: number, now = Date.now()) {
  const say = new Intl.RelativeTimeFormat(undefined, { numeric: "auto", style: "short" });
  const s = Math.round((at - now) / 1000);
  for (const [unit, size] of [
    ["day", 86400],
    ["hour", 3600],
    ["minute", 60],
  ] as const)
    if (Math.abs(s) >= size) return say.format(Math.round(s / size), unit);
  return say.format(0, "second");
}

/* SEARCH */

export const terms = (query: string) => query.toLowerCase().split(/\s+/).filter(Boolean);

export function score(words: string[], title: string, more = "") {
  const name = title.toLowerCase();
  const rest = more.toLowerCase();
  let total = 0;
  for (const word of words) {
    const at = name.indexOf(word);
    if (at === 0) total += 4;
    else if (at > 0 && /[^\p{L}\p{N}]/u.test(name[at - 1])) total += 3;
    else if (at > 0) total += 2;
    else if (word.length >= 3 && rest.includes(word)) total += 1;
    else return 0;
  }
  return total;
}

export function rank<T extends Ranked>(query: string, items: T[], cap = Infinity): T[] {
  const words = terms(query);
  if (!words.length) return [];
  return items
    .map((one): [T, number] => [one, score(words, one.title, one.more)])
    .filter(([, n]) => n > 0)
    .sort((a, b) => b[1] - a[1] || a[0].title.localeCompare(b[0].title))
    .slice(0, cap)
    .map(([one]) => one);
}

/* DAY */

export const today = (now = new Date()) =>
  `${now.getUTCFullYear()}-${two(now.getUTCMonth() + 1)}-${two(now.getUTCDate())}`;

export function daily(day: string, n: number) {
  let h = 0x811c9dc5;
  for (const c of day) h = Math.imul(h ^ c.charCodeAt(0), 0x01000193) >>> 0;
  return n > 0 ? h % n : 0;
}
