export const RECENT = 9;
export const KEPT = 24;
export const SPARE = 'New Section';

const two = (n) => String(n).padStart(2, '0');
const parse = (text) => {
  try {
    return text ? JSON.parse(text) : null;
  } catch {
    return null;
  }
};

/* LAYOUT */

export function arrange(base, text) {
  const saved = parse(text);
  const known = new Set(base.flatMap((one) => one.apps));
  const seen = new Set();
  const names = new Set();
  const out = [];
  for (const one of Array.isArray(saved) ? saved : []) {
    if (!one || typeof one.name !== 'string' || !one.name.trim() || !Array.isArray(one.apps) || names.has(one.name.toLowerCase())) continue;
    names.add(one.name.toLowerCase());
    out.push({ name: one.name, apps: one.apps.filter((id) => known.has(id) && !seen.has(id) && seen.add(id)) });
  }
  for (const shelf of base) {
    const lost = shelf.apps.filter((id) => !seen.has(id));
    if (!lost.length) continue;
    let home = out.find((one) => one.name.toLowerCase() === shelf.name.toLowerCase());
    if (!home) out.push((home = { name: shelf.name, apps: [] }));
    home.apps.push(...lost);
  }
  return out;
}

export function docked(base, text, known) {
  const saved = parse(text);
  const seen = new Set();
  const groups = (Array.isArray(saved) && saved.every(Array.isArray) ? saved : base).map((group) => group.filter((id) => known.has(id) && !seen.has(id) && seen.add(id)));
  while (groups.length < base.length) groups.push([]);
  return groups;
}

export function spare(shelves) {
  const taken = new Set(shelves.map((one) => one.name.toLowerCase()));
  let n = 1;
  while (taken.has((n === 1 ? SPARE : `${SPARE} ${n}`).toLowerCase())) n++;
  return n === 1 ? SPARE : `${SPARE} ${n}`;
}

export function renamed(shelves, from, to) {
  const name = to.trim();
  if (!name || shelves.some((one) => one.name !== from && one.name.toLowerCase() === name.toLowerCase())) return shelves;
  return shelves.map((one) => (one.name === from ? { ...one, name } : one));
}

const line = (ids) => `[${ids.map((id) => JSON.stringify(id)).join(', ')}]`;

export function exported(shelves, dock) {
  const rows = shelves.map((one) => `    { "name": ${JSON.stringify(one.name)}, "apps": ${line(one.apps)} }`);
  return `{\n  "sections": [\n${rows.join(',\n')}\n  ],\n  "dock": [${dock.map(line).join(', ')}]\n}\n`;
}

/* RECENT */

export const siblings = (shelves, id) => shelves.find((one) => one.apps.includes(id))?.apps.filter((one) => one !== id) ?? [];

export function visited(list, entry) {
  return [entry, ...(Array.isArray(list) ? list : []).filter((one) => one?.id && one.id !== entry.id)].slice(0, KEPT);
}

export const recents = (text, known) => (parse(text) ?? []).filter((one) => typeof one?.id === 'string' && known.has(one.id) && typeof one.url === 'string' && one.url.startsWith('/'));

export function gist(url) {
  const params = new URLSearchParams(url.split('?')[1] ?? '');
  return [...params].filter(([key, value]) => key !== 'seed' && value !== '').slice(0, 3).map(([key, value]) => `${key} ${value}`).join(' · ');
}

export function ago(at, now = Date.now()) {
  const say = new Intl.RelativeTimeFormat(undefined, { numeric: 'auto', style: 'short' });
  const s = Math.round((at - now) / 1000);
  for (const [unit, size] of [['day', 86400], ['hour', 3600], ['minute', 60]]) if (Math.abs(s) >= size) return say.format(Math.round(s / size), unit);
  return say.format(0, 'second');
}

/* SEARCH */

export const terms = (query) => query.toLowerCase().split(/\s+/).filter(Boolean);

export function score(words, title, more = '') {
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

export function rank(query, items, cap = Infinity) {
  const words = terms(query);
  if (!words.length) return [];
  return items
    .map((one) => [one, score(words, one.title, one.more)])
    .filter(([, n]) => n > 0)
    .sort((a, b) => b[1] - a[1] || a[0].title.localeCompare(b[0].title))
    .slice(0, cap)
    .map(([one]) => one);
}

/* DAY */

export const today = (now = new Date()) => `${now.getFullYear()}-${two(now.getMonth() + 1)}-${two(now.getDate())}`;

export function daily(day, n) {
  let h = 0x811c9dc5;
  for (const c of day) h = Math.imul(h ^ c.charCodeAt(0), 0x01000193) >>> 0;
  return n > 0 ? h % n : 0;
}
