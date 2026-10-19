import { useEffect, useMemo, useRef, useState } from 'react';
import { flushSync } from 'react-dom';
import { rows } from 'site:routes';
import { Icon, Menu } from '../lib/app/controls.jsx';
import { text } from '../lib/app/export.js';
import { placed, sortable } from '../lib/drag.js';
import { KNOWN, PAD, app, away, dock, notes, recent, redock, shelve, shelves as saved, take, unshelve } from './catalog.js';
import { RECENT, exported, rank, renamed, spare } from './launch.js';
import { Glyph, PANES } from './parts.jsx';
import { Tile } from './tile.jsx';

const CAP = { apps: 27, list: 6 };
const SAID = 5000;
const SEEN = ' recent';

const section = (id) => PAD.sections.find((one) => one.apps.includes(id))?.name ?? '';

const APPS = [...KNOWN].map((id) => ({ id, title: app(id).title, more: `${app(id).lead} ${section(id)}`, href: app(id).route }));

const POSTS = rows.filter((one) => one.meta?.prose === 'post' && !one.hidden).map((one) => ({ title: one.title, more: one.lead, href: one.route }));

let pull = null;

export function opened(host) {
  pull?.();
  const field = host.querySelector('.find input');
  if (!field || (!field.value && !matchMedia('(hover: hover)').matches)) return;
  field.focus({ preventScroll: true });
  field.setSelectionRange(field.value.length, field.value.length);
}

const without = (map, key) => Object.fromEntries(Object.entries(map).filter(([one]) => one !== key));

/* RENAME */

function Rename({ name, onDone, className }) {
  const done = useRef(false);
  const end = (value) => {
    if (done.current) return;
    done.current = true;
    onDone(value);
  };
  return (
    <input
      className={className}
      defaultValue={name}
      aria-label={`Rename ${name}`}
      autoFocus
      onFocus={(e) => e.target.select()}
      onBlur={(e) => end(e.target.value)}
      onKeyDown={(e) => {
        if (e.key === 'Enter') end(e.currentTarget.value);
        if (e.key === 'Escape') {
          e.preventDefault();
          e.stopPropagation();
          end(name);
        }
      }}
    />
  );
}

/* SHELF */

function Shelf({ slot, name, kind, ids, draft, scroller, naming, hooks }) {
  const list = useRef(null);
  useEffect(
    () =>
      sortable(list.current, {
        kind,
        takes: (from) => (kind === 'launchpad' && from === 'launchpad' ? 'move' : ''),
        scroller: scroller.current,
        name: (id) => app(id)?.title ?? id,
        preview: (next) => hooks.current.preview(slot, next),
        commit: (next) => hooks.current.commit(slot, next),
      }),
    [slot, kind],
  );
  const shown = placed(ids, draft);
  const held = draft && draft.index !== null ? draft.id : null;
  const head = `shelf-${slot || 'recent'}`.replace(/\W+/g, '-');
  return (
    <section className="shelf" aria-labelledby={head}>
      {naming ? <Rename className="name" name={name} onDone={(to) => hooks.current.rename(name, to)} /> : <h2 id={head} onDoubleClick={kind === 'launchpad' ? () => hooks.current.name(name) : undefined}>{name}</h2>}
      <ul className="apps" ref={list}>
        {shown.map((id) => (
          <li key={id} data-id={id} data-held={id === held ? '' : undefined}>
            <Tile id={id} />
          </li>
        ))}
      </ul>
      {!shown.length && kind === 'launchpad' && (
        <p className="empty">
          Drag apps here, or{' '}
          <button type="button" className="link" onClick={() => hooks.current.drop(name)}>
            remove this section
          </button>
          .
        </p>
      )}
    </section>
  );
}

/* CHIPS */

function Chips({ names, chip, naming, hooks }) {
  const box = useRef(null);
  const [draft, setDraft] = useState(null);
  useEffect(
    () =>
      sortable(box.current, {
        kind: 'sections',
        axis: 'x',
        takes: (from) => (from === 'sections' ? 'move' : ''),
        preview: (next) => flushSync(() => setDraft(next)),
        commit: (next) => {
          flushSync(() => setDraft(null));
          if (next) hooks.current.order(next);
        },
      }),
    [],
  );
  return (
    <nav className="chips" aria-label="Sections">
      <button type="button" className="pill" aria-pressed={chip === ''} onClick={() => hooks.current.chip('')}>
        All
      </button>
      <div className="names" ref={box}>
        {placed(names, draft).map((name) =>
          naming === name ? (
            <Rename key={name} className="pill" name={name} onDone={(to) => hooks.current.rename(name, to)} />
          ) : (
            <button type="button" key={name} className="pill" data-id={name} data-held={draft?.id === name && draft.index !== null ? '' : undefined} aria-pressed={chip === name} onClick={() => hooks.current.chip(chip === name ? '' : name)} onDoubleClick={() => hooks.current.name(name)}>
              {name}
            </button>
          ),
        )}
      </div>
      <button type="button" className="pill add" aria-label="New section" title="New section" onClick={() => hooks.current.add()}>
        <Glyph art="plus" />
      </button>
    </nav>
  );
}

/* RESULTS */

function Results({ groups, at, query }) {
  let n = 0;
  const [apps, ...lists] = groups;
  const none = groups.every(([, list]) => !list.length);
  if (none) return <p className="none">Nothing found for {query.trim()}.</p>;
  return (
    <div className="results" id="hits">
      {apps[1].length > 0 && (
        <section className="shelf" aria-label={apps[0]}>
          <h2>{apps[0]}</h2>
          <ul className="apps">
            {apps[1].map((one) => {
              const i = n++;
              return (
                <li key={one.id} id={`hit-${i}`} className={i === at ? 'on' : undefined}>
                  <Tile id={one.id} />
                </li>
              );
            })}
          </ul>
        </section>
      )}
      <div className="lists">
        {lists.map(
          ([name, list]) =>
            list.length > 0 && (
              <section key={name} aria-label={name}>
                <h2>{name}</h2>
                <ul className="hits">
                  {list.map((one) => {
                    const i = n++;
                    return (
                      <li key={one.href} id={`hit-${i}`} className={i === at ? 'on' : undefined}>
                        <a href={one.href}>{one.title}</a>
                      </li>
                    );
                  })}
                </ul>
              </section>
            ),
        )}
      </div>
    </div>
  );
}

/* LAUNCHPAD */

export function Launchpad() {
  const [shelves, setShelves] = useState(saved);
  const [drafts, setDrafts] = useState({});
  const [chip, setChip] = useState('');
  const [query, setQuery] = useState(() => take() || new URLSearchParams(location.search).get('q') || '');
  const [hit, setHit] = useState(0);
  const [naming, setNaming] = useState('');
  const [said, setSaid] = useState(null);
  const [found, setFound] = useState([]);
  const [seen, setSeen] = useState(recent);
  const live = useRef(shelves);
  const root = useRef(null);
  const body = useRef(null);
  const field = useRef(null);
  const hooks = useRef(null);
  const searching = query.trim() !== '';

  const keep = (next) => {
    live.current = next;
    shelve(next);
    setShelves(next);
  };
  const tell = (words, undo) => {
    setSaid({ words, undo });
    clearTimeout(tell.timer);
    tell.timer = setTimeout(() => setSaid(null), SAID);
  };

  hooks.current = {
    preview: (slot, next) => flushSync(() => setDrafts((was) => ({ ...was, [slot]: next }))),
    commit: (slot, next) =>
      flushSync(() => {
        setDrafts((was) => without(was, slot));
        if (next && slot !== SEEN) keep(live.current.map((one) => (one.name === slot ? { ...one, apps: placed(one.apps, next) } : one)));
      }),
    order: (next) => {
      const by = new Map(live.current.map((one) => [one.name, one]));
      keep(placed(live.current.map((one) => one.name), next).map((name) => by.get(name)));
    },
    chip: setChip,
    name: setNaming,
    rename: (from, to) => {
      const next = renamed(live.current, from, to);
      setNaming('');
      if (next === live.current) return;
      keep(next);
      if (chip === from) setChip(to.trim());
    },
    drop: (name) => {
      keep(live.current.filter((one) => one.name !== name));
      if (chip === name) setChip('');
    },
    add: () => {
      const name = spare(live.current);
      keep([...live.current, { name, apps: [] }]);
      setChip('');
      setNaming(name);
    },
  };

  const save = () => {
    const json = exported(live.current, dock());
    text(json, 'launchpad', 'json');
    navigator.clipboard?.writeText(json).catch(() => {});
    tell('Saved launchpad.json and copied it.');
  };
  const reset = () => {
    const was = { shelves: live.current, dock: dock() };
    unshelve();
    setChip('');
    tell('Layout reset.', () => {
      keep(was.shelves);
      redock(was.dock);
      setSaid(null);
    });
  };

  const groups = useMemo(
    () =>
      searching
        ? [
            ['Apps', rank(query, APPS, CAP.apps)],
            ['Research', rank(query, found, CAP.list)],
            ['Posts', rank(query, POSTS, CAP.list)],
            ['Settings', rank(query, PANES, CAP.list)],
          ]
        : [],
    [query, found, searching],
  );
  const hits = groups.flatMap(([, list]) => list);
  const at = hits.length ? Math.min(hit, hits.length - 1) : -1;
  const ids = [...new Set(seen.map((one) => one.id))].slice(0, RECENT);

  useEffect(() => {
    const fresh = () => setSeen(recent());
    const reload = () => {
      live.current = saved();
      setShelves(live.current);
    };
    const press = (e) => {
      const target = e.target instanceof Element ? e.target : null;
      if (!target || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
      if (target.closest('.trio .launch, .dock .launch')) {
        e.preventDefault();
        e.stopPropagation();
        away();
      } else if (target.closest('.tray .find')) {
        e.preventDefault();
        e.stopPropagation();
        field.current?.focus();
      }
    };
    const key = (e) => {
      if (e.key !== 'Escape' || e.defaultPrevented || document.querySelector(':popover-open, dialog[open]')) return;
      e.preventDefault();
      away();
    };
    addEventListener('recent', fresh);
    addEventListener('shelves', reload);
    document.addEventListener('click', press, true);
    document.addEventListener('keydown', key);
    pull = () => {
      const more = take();
      if (more) flushSync(() => setQuery((was) => was + more));
    };
    return () => {
      pull = null;
      removeEventListener('recent', fresh);
      removeEventListener('shelves', reload);
      document.removeEventListener('click', press, true);
      document.removeEventListener('keydown', key);
      clearTimeout(tell.timer);
    };
  }, []);

  useEffect(() => {
    if (searching) notes().then(setFound);
  }, [searching]);

  useEffect(() => {
    dispatchEvent(new CustomEvent('shift', { detail: 0 }));
  });

  useEffect(() => {
    if (at >= 0) root.current?.querySelector(`#hit-${at}`)?.scrollIntoView({ block: 'nearest' });
  }, [at]);

  const press = (e) => {
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      if (query) setQuery('');
      else away();
      return;
    }
    if (!searching || !hits.length) return;
    const by = e.key === 'Tab' ? (e.shiftKey ? -1 : 1) : e.key === 'ArrowDown' ? 1 : e.key === 'ArrowUp' ? -1 : 0;
    if (by) {
      e.preventDefault();
      setHit((at + by + hits.length) % hits.length);
      return;
    }
    if (e.key === 'Enter') {
      e.preventDefault();
      root.current?.querySelector(`#hit-${at} a`)?.click();
    }
  };

  return (
    <div className="launchpad" ref={root}>
      <div className="veil" onClick={away} aria-hidden="true" />
      <section className="pad" data-lens="launchpad" aria-label="Launchpad">
        <header>
          <Glyph art="apps" />
          {!searching && <h1>Apps</h1>}
          <form role="search" className="find" onSubmit={(e) => e.preventDefault()}>
            <Glyph art="find" />
            <input
              ref={field}
              type="search"
              value={query}
              placeholder="Search apps, notes, settings"
              aria-label="Search"
              role="combobox"
              aria-expanded={searching}
              aria-controls="hits"
              aria-activedescendant={at >= 0 ? `hit-${at}` : undefined}
              autoComplete="off"
              spellCheck={false}
              enterKeyHint="go"
              onChange={(e) => {
                setQuery(e.target.value);
                setHit(0);
              }}
              onKeyDown={press}
            />
          </form>
          <Menu
            className="icon more"
            aria-label="More"
            rows={[
              ['New Section', () => hooks.current.add()],
              ['Export JSON', save, 'download'],
              ['Reset Layout', reset],
            ]}
          >
            <Icon name="more_horiz" />
          </Menu>
        </header>
        {!searching && <Chips names={shelves.map((one) => one.name)} chip={chip} naming={naming} hooks={hooks} />}
        <div className="body" ref={body}>
          {searching ? (
            <Results groups={groups} at={at} query={query} />
          ) : (
            <>
              {chip === '' && ids.length > 0 && <Shelf slot={SEEN} name="Recent" kind="recent" ids={ids} draft={drafts[SEEN]} scroller={body} hooks={hooks} />}
              {shelves
                .filter((one) => !chip || one.name === chip)
                .map((one) => (
                  <Shelf key={one.name} slot={one.name} name={one.name} kind="launchpad" ids={one.apps} draft={drafts[one.name]} scroller={body} naming={naming === one.name && chip !== ''} hooks={hooks} />
                ))}
            </>
          )}
        </div>
        {said && (
          <p className="said" role="status">
            {said.words}
            {said.undo && (
              <button type="button" className="link" onClick={said.undo}>
                Undo
              </button>
            )}
          </p>
        )}
      </section>
    </div>
  );
}
