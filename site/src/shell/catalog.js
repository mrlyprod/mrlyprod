import { rows, site } from 'site:routes';
import { arrange, docked, recents, visited } from './launch.js';
import { KEY, read, write } from './prefs.js';

export const PAD = site.launchpad;
export const KNOWN = new Set(PAD.sections.flatMap((one) => one.apps));

const BY = new Map(rows.filter((one) => one.meta?.id).map((one) => [one.meta.id, one]));

export const app = (id) => BY.get(id);
export const tell = (name) => dispatchEvent(new Event(name));

/* SHELVES */

export const shelves = () => arrange(PAD.sections, read(KEY.launchpad));

export function shelve(next) {
  write(KEY.launchpad, JSON.stringify(next));
  tell('shelves');
}

/* DOCK */

export const dock = () => docked(PAD.dock, read(KEY.dock), KNOWN);

export function redock(next) {
  write(KEY.dock, JSON.stringify(next));
  tell('dock');
}

export function unshelve() {
  write(KEY.launchpad, '');
  write(KEY.dock, '');
  tell('shelves');
  tell('dock');
}

/* RECENT */

export const recent = () => recents(read(KEY.recent), KNOWN);

let open = '';
let at = '';

function note() {
  write(KEY.recent, JSON.stringify(visited(recent(), { id: open, url: location.pathname + location.search, at: Date.now() })));
  tell('recent');
}

export function track() {
  addEventListener('wire', (e) => {
    if (!e.detail) return;
    const one = rows.find((row) => row.route === e.detail?.route);
    open = one && KNOWN.has(one.meta?.id) ? one.meta.id : '';
    at = open ? one.route : '';
    if (open) note();
  });
  addEventListener('stamp', () => open && location.pathname.startsWith(at) && note());
}

/* NOTES */

let topics = null;

export const notes = () =>
  (topics ??= fetch('/research.json')
    .then((reply) => (reply.ok ? reply.json() : Promise.reject(new Error(`research: /research.json answered ${reply.status}`))))
    .then((data) => (data.topics ?? []).map((one) => ({ ...one, more: one.lead ?? '', href: `/research/${encodeURIComponent(one.slug)}/` })))
    .catch(() => {
      topics = null;
      return [];
    }));

/* LAUNCH */

let ahead = '';

export const type = (key) => {
  ahead += key;
};

export const take = () => {
  const out = ahead;
  ahead = '';
  return out;
};

export const away = () => (history.state?.depth > 0 ? history.back() : dispatchEvent(new CustomEvent('go', { detail: '/' })));
