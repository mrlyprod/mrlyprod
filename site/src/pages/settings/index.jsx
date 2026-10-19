import { useEffect, useRef, useState } from 'react';
import { flushSync } from 'react-dom';
import { Icon, Menu } from '../../lib/app/controls.jsx';
import { text } from '../../lib/app/export.js';
import { rank, terms } from '../../shell/launch.js';
import { hide, show } from '../../shell/parts.jsx';
import { decode, pack, share, unpack } from '../../shell/prefs.js';
import { named } from '../../shell/route.js';
import About, { FIND as about } from './about.jsx';
import Accessibility, { FIND as accessibility } from './accessibility.jsx';
import Appearance, { FIND as appearance } from './appearance.jsx';
import Desktop, { FIND as desktop } from './desktop.jsx';
import Glass, { FIND as glass } from './glass.jsx';
import { PANES } from '../../shell/panes.js';
import Privacy, { FIND as privacy } from './privacy.jsx';
import Screensaver, { FIND as screensaver } from './screensaver.jsx';
import { useForm } from './use.js';

export const layout = { bare: true };

/* PANES */

const ROOT = '/settings/';
const FIRST = 'appearance';
const SAID = 2500;
const SHOWN = 3;

const VIEWS = { appearance: Appearance, glass: Glass, desktop: Desktop, screensaver: Screensaver, accessibility: Accessibility, privacy: Privacy, about: About };
const FINDS = { appearance, glass, desktop, screensaver, accessibility, privacy, about };
const IDS = new Set(PANES.map((one) => one.id));
const ITEMS = PANES.map((one) => ({ ...one, more: [one.more, ...(FINDS[one.id] ?? [])].join(' ') }));

function spot(path) {
  const id = path.startsWith(ROOT) ? path.slice(ROOT.length).split('/')[0] : '';
  return IDS.has(id) ? `${ROOT}${id}/` : ROOT;
}

/* PAGE */

const state = () => (history.state && typeof history.state === 'object' ? history.state : {});
const main = () => document.getElementById('main');
const roll = (top) => main()?.scrollTo({ top, behavior: 'instant' });

function head(row, site, path) {
  const pane = PANES.find((one) => one.href === path);
  const base = named(row, site);
  document.title = pane ? `${pane.title} · ${base}` : base;
  document.querySelector('link[rel="canonical"]')?.setAttribute('href', site.root + path);
}

function refocus() {
  const now = document.activeElement;
  if (!now || now === document.body || !now.isConnected) main()?.focus({ preventScroll: true });
}

/* SIDEBAR */

function Square({ art, hue }) {
  const rows = art.split(' ');
  return (
    <span className="symbol" style={{ '--hue': `var(--${hue})` }} aria-hidden="true">
      <svg className="glyphs" viewBox={`0 0 ${rows[0].length} ${rows.length}`} style={{ '--rows': rows.length, '--cols': rows[0].length }}>
        {rows.flatMap((line, y) => [...line].map((c, x) => (c === '#' ? <rect key={`${x}:${y}`} x={x} y={y} width="1" height="1" /> : null)))}
      </svg>
    </span>
  );
}

function Row({ one, on, labels, phone }) {
  return (
    <li>
      <a href={one.href} aria-current={on ? 'page' : undefined}>
        <Square art={one.art} hue={one.hue} />
        <span className="words">
          <span className="name">{one.title}</span>
          {labels.length > 0 && <span className="hint">{labels.join(', ')}</span>}
        </span>
        {phone && <Icon name="chevron_right" size={16} />}
      </a>
    </li>
  );
}

/* LINK */

function Offer({ code, drop }) {
  const [broken, setBroken] = useState(false);
  const data = decode(code);
  const apply = () => {
    try {
      unpack(data);
      location.replace(ROOT);
    } catch {
      setBroken(true);
    }
  };
  if (!data || broken) {
    return (
      <div className="banner" role="alert">
        <p>
          <strong>This settings link is broken.</strong>
          <span>It may have been cut short.</span>
        </p>
        <span className="pair">
          <button type="button" className="push" onClick={drop}>OK</button>
        </span>
      </div>
    );
  }
  return (
    <div className="banner" role="region" aria-label="Settings link">
      <p>
        <strong>Apply settings from this link?</strong>
        <span>They replace the settings kept in this browser.</span>
      </p>
      <span className="pair">
        <button type="button" className="push" onClick={drop}>Not now</button>
        <button type="button" className="push main" onClick={apply}>Apply</button>
      </span>
    </div>
  );
}

/* SETTINGS */

function Settings({ row, site, start }) {
  const phone = useForm() === 'phone';
  const [here, setHere] = useState(() => spot(start.pathname) + start.search);
  const [query, setQuery] = useState('');
  const [said, setSaid] = useState('');
  const box = useRef(null);
  const field = useRef(null);
  const list = useRef(null);
  const file = useRef(null);
  const timer = useRef(0);
  const live = useRef(null);

  const cut = here.indexOf('?');
  const path = cut < 0 ? here : here.slice(0, cut);
  const code = cut < 0 ? null : new URLSearchParams(here.slice(cut)).get('apply');
  const id = path.slice(ROOT.length, -1);
  const shown = id || (phone ? '' : FIRST);
  const View = VIEWS[shown];
  const words = terms(query);
  const hits = words.length ? rank(query, ITEMS) : ITEMS;
  const found = (one) => (words.length ? (FINDS[one.id] ?? []).filter((label) => words.every((word) => label.toLowerCase().includes(word))).slice(0, SHOWN) : []);

  const open = (to, how = 'push') => {
    const next = new URL(to, location.href);
    const fresh = spot(next.pathname);
    const href = fresh + next.search;
    if (how === 'push') {
      history.replaceState({ ...state(), y: Math.round(main()?.scrollTop ?? 0) }, '');
      const was = state();
      history.pushState({ ...was, y: 0, depth: (was.depth ?? 0) + 1, up: path === ROOT }, '', href);
    } else history.replaceState(state(), '', href);
    flushSync(() => setHere(href));
    roll(0);
    head(row, site, fresh);
    refocus();
  };

  const pop = () => {
    const fresh = spot(location.pathname);
    flushSync(() => setHere(fresh + location.search));
    roll(typeof state().y === 'number' ? state().y : 0);
    head(row, site, fresh);
    refocus();
  };

  live.current = { open, pop };

  useEffect(() => {
    const mine = () => Boolean(box.current?.isConnected);
    const click = (e) => {
      if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey || !mine()) return;
      const a = e.target instanceof Element ? e.target.closest('a[href]') : null;
      if (!(a instanceof HTMLAnchorElement) || a.target || a.hasAttribute('download') || a.origin !== location.origin || !a.pathname.startsWith(ROOT)) return;
      e.preventDefault();
      live.current.open(a.pathname + a.search, a.href === location.href ? 'replace' : 'push');
    };
    const pop = () => {
      if (mine() && location.pathname.startsWith(ROOT)) live.current.pop();
    };
    const key = (e) => {
      if (e.defaultPrevented || e.altKey || typeof e.key !== 'string' || !mine() || !field.current) return;
      const typing = e.target instanceof Element && e.target.closest('input, textarea, select, [contenteditable]');
      const slash = e.key === '/' && !e.metaKey && !e.ctrlKey && !typing;
      const find = (e.metaKey || e.ctrlKey) && !e.shiftKey && e.key.toLowerCase() === 'f';
      if ((!slash && !find) || document.querySelector('dialog[open]')) return;
      e.preventDefault();
      field.current.focus();
      field.current.select();
    };
    document.addEventListener('click', click);
    document.addEventListener('keydown', key);
    addEventListener('popstate', pop);
    return () => {
      document.removeEventListener('click', click);
      document.removeEventListener('keydown', key);
      removeEventListener('popstate', pop);
      clearTimeout(timer.current);
    };
  }, []);

  const tell = (words) => {
    clearTimeout(timer.current);
    setSaid(words);
    timer.current = setTimeout(() => setSaid(''), SAID);
  };

  const copy = () => {
    const link = share(location.origin);
    const done = navigator.clipboard ? navigator.clipboard.writeText(link) : Promise.reject(new Error('no clipboard'));
    done.then(
      () => tell('Link copied'),
      () => tell('The link could not be copied'),
    );
  };

  const keep = () => text(JSON.stringify(pack(), null, 2), 'mrly-settings', 'json');

  const load = async (e) => {
    const one = e.target.files?.[0];
    e.target.value = '';
    if (!one) return;
    try {
      unpack(JSON.parse(await one.text()));
      location.reload();
    } catch {
      tell('That file holds no settings');
    }
  };

  const drop = () => {
    history.replaceState(state(), '', path);
    setHere(path);
  };

  const press = (e) => {
    if (e.key === 'Escape' && query) {
      e.preventDefault();
      e.stopPropagation();
      setQuery('');
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (words.length && hits.length) open(hits[0].href);
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      list.current?.querySelector('a')?.focus();
    }
  };

  const walk = (e) => {
    const links = [...(list.current?.querySelectorAll('a') ?? [])];
    const at = links.indexOf(document.activeElement);
    const to = e.key === 'ArrowDown' ? at + 1 : e.key === 'ArrowUp' ? at - 1 : e.key === 'Home' ? 0 : e.key === 'End' ? links.length - 1 : null;
    if (at < 0 || to === null) return;
    e.preventDefault();
    if (to < 0) field.current?.focus();
    else links[Math.min(to, links.length - 1)].focus();
  };

  const back = (e) => {
    if (!state().up) return;
    e.preventDefault();
    history.back();
  };

  const side = !phone || !id;
  const pane = Boolean(View);
  const offer = code !== null && <Offer key={code} code={code} drop={drop} />;

  return (
    <div className={side && pane ? 'settings two' : 'settings'} ref={box}>
      {side && (
        <div className="side">
          {phone && <h1>{row.title}</h1>}
          {!pane && offer}
          <div className="top">
            <form role="search" className="find" onSubmit={(e) => e.preventDefault()}>
              <Icon name="search" size={16} />
              <input ref={field} type="search" value={query} placeholder="Search" aria-label="Search settings" autoComplete="off" spellCheck={false} enterKeyHint="go" onChange={(e) => setQuery(e.target.value)} onKeyDown={press} />
            </form>
            <Menu
              className="icon"
              aria-label="More"
              rows={[
                ['Copy settings link', copy, 'link'],
                ['Export settings', keep, 'download'],
                ['Import settings…', () => file.current?.click()],
              ]}
            >
              <Icon name="more_horiz" />
            </Menu>
            <input ref={file} type="file" accept="application/json,.json" hidden onChange={load} />
            <p className="said" role="status">{said}</p>
          </div>
          <nav aria-label="Settings">
            {hits.length ? (
              <ul className="list" ref={list} onKeyDown={walk}>
                {hits.map((one) => <Row key={one.id} one={one} on={one.id === shown} labels={found(one)} phone={phone} />)}
              </ul>
            ) : (
              <p className="none">No results</p>
            )}
          </nav>
        </div>
      )}
      {pane && (
        <div className="detail">
          {offer}
          {phone && (
            <a className="back" href={ROOT} onClick={back}>
              <Icon name="chevron_right" size={20} />
              {row.title}
            </a>
          )}
          <View key={shown} site={site} />
        </div>
      )}
    </div>
  );
}

/* ROUTE */

const pages = new WeakMap();

export function render(row, site, host, { url }) {
  pages.set(host, { row, site });
  show(host, <Settings row={row} site={site} start={url} />);
}

export function mount(host) {
  const page = pages.get(host);
  if (!page || !location.pathname.startsWith(ROOT)) return;
  const path = spot(location.pathname);
  if (location.pathname !== path) history.replaceState(history.state, '', path + location.search + location.hash);
  head(page.row, page.site, path);
}

export function unmount(host) {
  hide(host);
  pages.delete(host);
}
