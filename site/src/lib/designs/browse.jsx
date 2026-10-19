import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Btn, Icon, Segment } from '../app/controls.jsx';
import { cap, facts, gallery, home, total } from './engine.js';
import { Thumb, useTheme, watch } from './pick.jsx';
import { allows, bases, dims, fits, read, same, useDesigns } from './store.js';

export const SHOWS = [['classes', 'Classes'], ['named', 'Named'], ['all', 'All']];

const CHUNK = 240;
const CAP = 120;
const SMALL = 56;
const LARGE = 96;
const BIG = 320;
const DEEP = 4;

/* WORDS */

const cache = new Map();

const kept = (kind, { dim, base, code }, fn) => {
  const key = `${kind}:${dim}:${base}:${code}`;
  if (!cache.has(key)) cache.set(key, fn());
  return cache.get(key);
};

const size = (math, dim, base) => kept('total', { dim, base, code: '' }, () => total(math, dim, base));

export const valid = (math, design) => Boolean(math && design) && /^\d+$/.test(String(design.code)) && Number(design.code) < size(math, Number(design.dim), Number(design.base));

export const called = (math, design) => (valid(math, design) ? kept('name', design, () => math.name.of(String(design.code), Number(design.dim), Number(design.base)) ?? '') : '');

const form = (math, design) => (Number(design.base) === 2 && valid(math, design) ? kept('anf', design, () => facts(math, Number(design.dim), 2, String(design.code)).anf) : '');

export function named(math, design) {
  const code = String(design.code);
  if (!math) return code;
  const name = called(math, design);
  return name ? `${name} ${code}` : `code ${code}`;
}

export function here(slot, value, math) {
  const one = read(slot, value);
  return one.code || !math ? one : { ...one, code: home(math, one.dim, one.base) };
}

export function about(math, design) {
  const rows = [['Code', String(design.code)], ['Space', `${design.dim}D, base ${design.base}`]];
  if (!valid(math, design)) return rows;
  const { rep, orbit, anf } = facts(math, Number(design.dim), Number(design.base), String(design.code));
  return [...rows, ['Class', String(rep)], ['Orbit', String(orbit)], ...(anf ? [['Normal form', anf]] : [])];
}

/* SEARCH */

export const shows = (math, dim, base) => SHOWS.filter(([one]) => one === 'named' || gallery(math, dim, base, one) !== null);

export function matches(math, design, query) {
  const want = query.trim().toLowerCase();
  return !want || String(design.code).startsWith(want) || called(math, design).toLowerCase().includes(want) || form(math, design).toLowerCase().includes(want);
}

export function search(math, space, list, query) {
  const hits = list.filter((one) => matches(math, one, query));
  const want = query.trim();
  const exact = /^\d+$/.test(want) ? { dim: space.dim, base: space.base, code: String(Number(want)) } : null;
  return exact && valid(math, exact) ? [exact, ...hits.filter((one) => !same(one, exact))] : hits;
}

/* KEYS */

export function step(at, key, cols, count) {
  const to = { ArrowLeft: at - 1, ArrowRight: at + 1, ArrowUp: at - cols, ArrowDown: at + cols, Home: 0, End: count - 1 }[key];
  if (to === undefined) return -1;
  return to < 0 || to >= count ? at : to;
}

const across = (grid) => (typeof getComputedStyle === 'function' ? getComputedStyle(grid).gridTemplateColumns.split(' ').filter(Boolean).length : 0) || 1;

function arrows(e) {
  const cell = e.target.closest?.('.designs > button');
  if (!cell) return;
  const cells = [...cell.parentElement.children];
  const to = step(cells.indexOf(cell), e.key, across(cell.parentElement), cells.length);
  if (to < 0) return;
  e.preventDefault();
  e.stopPropagation();
  cells[to].focus();
}

/* CELLS */

function Cell({ math, design, on, px, tick, onPick, onLook }) {
  const label = named(math, design);
  const look = onLook && (() => onLook(design));
  return (
    <button type="button" className={on ? 'on' : undefined} aria-pressed={on} aria-label={label} title={`${label}, ${design.dim}D base ${design.base}`} onClick={() => onPick(design)} onFocus={look} onPointerEnter={look}>
      <Thumb math={math} dim={design.dim} base={design.base} code={design.code} px={px} tick={tick} />
      <small>{called(math, design) || design.code}</small>
    </button>
  );
}

function Grid({ label, designs, now, ...rest }) {
  return (
    <div className="designs" role="group" aria-label={label}>
      {designs.map((one) => <Cell key={`${one.dim}:${one.base}:${one.code}`} design={one} on={same(one, now)} {...rest} />)}
    </div>
  );
}

function Shelf({ name, count, children }) {
  return (
    <section className="shelf" aria-label={name}>
      <h3>
        {name}
        {count !== undefined && <span>{count.toLocaleString()}</span>}
      </h3>
      {children}
    </section>
  );
}

function More({ left, onMore }) {
  const at = useRef(null);
  useEffect(() => watch(at.current, onMore), [onMore]);
  return <button type="button" className="more" ref={at} onClick={onMore}>{`${left.toLocaleString()} more`}</button>;
}

/* PREVIEW */

function Preview({ math, now, hold, tick, onPick }) {
  const [look, setLook] = useState(null);
  hold.current = setLook;
  const one = look ?? now;
  const ok = valid(math, one);
  return (
    <aside className="view" aria-label="Preview">
      {ok && <Thumb math={math} dim={one.dim} base={one.base} code={one.code} px={BIG} level={Math.min(cap(math, one.dim), DEEP)} tick={tick} />}
      <h3>{called(math, one) || `code ${one.code}`}</h3>
      <dl>
        {about(math, one).map(([name, text]) => (
          <div key={name}>
            <dt>{name}</dt>
            <dd>{text}</dd>
          </div>
        ))}
      </dl>
      <Btn primary disabled={!ok} onClick={() => onPick(one)}>Pick</Btn>
    </aside>
  );
}

/* BROWSER */

export function Browser({ slot, value, math, onPick, compact }) {
  const tick = useTheme();
  const { recent, saved } = useDesigns();
  const now = here(slot, value, math);
  const where = `${now.dim}:${now.base}`;
  const [space, setSpace] = useState({ where, dim: now.dim, base: now.base, show: '' });
  if (space.where !== where) setSpace({ ...space, where, dim: now.dim, base: now.base });
  const [query, setQuery] = useState('');
  const [grown, setGrown] = useState({ of: null, count: CHUNK });
  const hold = useRef(null);
  const list = useRef(null);
  const { dim, base } = space;
  const options = useMemo(() => (math ? shows(math, dim, base) : []), [math, dim, base]);
  const show = options.some(([one]) => one === space.show) ? space.show : options[0]?.[0];
  const holds = (one) => !slot.holds || slot.holds(one, value);
  const every = useMemo(() => (math ? (gallery(math, dim, base, show) ?? []).map((code) => ({ dim, base, code })).filter(holds) : []), [math, dim, base, show, slot.holds ? JSON.stringify(value) : '']);
  const found = useMemo(() => (math ? search(math, { dim, base }, every, query) : []), [math, dim, base, every, query]);
  const count = compact ? CAP : grown.of === found ? grown.count : CHUNK;
  const more = useCallback(() => setGrown({ of: found, count: count + CHUNK }), [found, count]);
  const look = useCallback((one) => hold.current?.(one), []);
  if (!math) return <div className="browser"><p className="none">Loading designs</p></div>;
  const shelf = (designs) => designs.filter((one) => fits(slot, one) && holds(one) && valid(math, one) && matches(math, one, query));
  const starred = shelf(saved);
  const lately = shelf(recent);
  const first = [...starred, ...lately, ...found][0];
  const cell = { math, now, px: compact ? SMALL : LARGE, tick, onPick, onLook: compact ? undefined : look };
  const move = (patch) => setSpace((was) => {
    const next = { ...was, ...patch };
    return allows(slot, next.dim, next.base) ? next : { ...next, base: bases(slot).find((one) => allows(slot, next.dim, one)) ?? next.base };
  });
  const spaces = dims(slot);
  const kinds = bases(slot).filter((one) => allows(slot, dim, one));
  const submit = (e) => {
    e.preventDefault();
    if (first) onPick(first);
  };
  const down = (e) => {
    const at = e.key === 'ArrowDown' && list.current?.querySelector('.designs > button');
    if (!at) return;
    e.preventDefault();
    at.focus();
  };
  return (
    <div className={compact ? 'browser compact' : 'browser'}>
      <form role="search" className="find" onSubmit={submit}>
        <Icon name="search" />
        <input type="search" value={query} placeholder="Name, code or form" aria-label="Search designs" autoComplete="off" spellCheck={false} enterKeyHint="go" onChange={(e) => setQuery(e.target.value)} onKeyDown={down} />
      </form>
      <div className="list" ref={list} onKeyDown={arrows}>
        {starred.length > 0 && (
          <Shelf name="Favorites">
            <Grid label="Favorites" designs={starred} {...cell} />
          </Shelf>
        )}
        {lately.length > 0 && (
          <Shelf name="Recent">
            <Grid label="Recent" designs={lately} {...cell} />
          </Shelf>
        )}
        <Shelf name={`${dim}D base ${base}`} count={found.length}>
          {(spaces.length > 1 || kinds.length > 1 || options.length > 1) && (
            <div className="space">
              {spaces.length > 1 && <Segment label="Dim" value={dim} options={spaces.map((one) => [one, `${one}D`])} onChange={(v) => move({ dim: v })} />}
              {kinds.length > 1 && <Segment label="Base" value={base} options={kinds.map((one) => [one, String(one)])} onChange={(v) => move({ base: v })} />}
              {options.length > 1 && <Segment label="Show" value={show} options={options} onChange={(v) => move({ show: v })} />}
            </div>
          )}
          {found.length > 0 ? <Grid label={`${dim}D base ${base}`} designs={found.slice(0, count)} {...cell} /> : <p className="none">No design matches</p>}
          {found.length > count && (compact ? <p className="none">{`${(found.length - count).toLocaleString()} more in Browse all`}</p> : <More left={found.length - count} onMore={more} />)}
        </Shelf>
      </div>
      {!compact && <Preview math={math} now={now} hold={hold} tick={tick} onPick={onPick} />}
    </div>
  );
}
