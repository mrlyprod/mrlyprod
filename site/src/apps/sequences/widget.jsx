import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { text } from '../../lib/app/export.js';
import { Bar } from '../../lib/app/frame.jsx';
import { rng } from '../../lib/rand.js';
import { Scene } from '../../lib/app/page.jsx';
import { KINDS } from '../../lib/sequences/chart.js';
import { DEEP, MOST, applies, csv, filter, identify, json, keyed, ledger, lead, measureOf, numbers, parse, picks, plotted, search } from '../../lib/sequences/engine.js';
import { RECORDS, facts, hit } from '../../lib/sequences/facts.js';
import { glyph, title } from '../../lib/sequences/paint.js';
import { Found, Record, shift } from '../../lib/sequences/parts.jsx';
import { make, named } from './scene.js';
import { useUnit } from '../../lib/app/hooks.js';

const SLICE = 8;
const PULSE = 90;
const PEEK = 6;
const SIDE = 3;
const THUMB = 1.25;
const PER = 50;

const roll = (seed, n) => Math.floor(rng(seed >>> 0)() * n);

const terse = (terms, count) => terms.slice(0, count).join(', ') + (terms.length > count ? ', …' : '');

const tone = (i, m) => `color-mix(in srgb, var(--accent) ${Math.round(100 - (m > 1 ? (62 * i) / (m - 1) : 0))}%, var(--art))`;

const ROW = { display: 'block', width: '100%', minHeight: 0, padding: '6px 8px', background: 'none', border: 0, borderLeft: '3px solid transparent', borderRadius: 0, textAlign: 'left', lineHeight: 1.4 };

const LINE = { display: 'block', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' };

const CHIP = { marginRight: 6, fontSize: 'var(--t1)', textTransform: 'lowercase' };

const STATUS = { Proved: 'var(--green)', Verified: 'var(--teal)', Collision: 'var(--orange)' };

function Thumb({ cells }) {
  const shape = glyph(cells);
  if (!shape) return null;
  return (
    <svg viewBox={`0 0 ${shape.width} ${SIDE}`} height={`${THUMB}em`} width={`${(THUMB * shape.width) / SIDE}em`} fill="currentColor" aria-hidden="true" style={{ flex: 'none', alignSelf: 'center', marginLeft: 6 }}>
      {cells.map((v, i) => <rect key={i} x={Math.floor(i / (SIDE * SIDE)) * (SIDE + 1) + (i % SIDE) + 0.08} y={(Math.floor(i / SIDE) % SIDE) + 0.08} width="0.84" height="0.84" opacity={v ? 1 : 0.15} />)}
    </svg>
  );
}

const Row = ({ row, hit, on, tone, typed, last, choose, add }) => (
  <li style={{ display: 'flex', alignItems: 'stretch', color: on ? 'var(--fg)' : 'var(--dim)' }}>
    <Thumb cells={row.cells} />
    <button type="button" aria-pressed={on} onClick={choose} style={{ ...ROW, flex: 1, minWidth: 0, borderLeftColor: on ? tone : 'transparent', color: 'inherit' }}>
      <span style={LINE}>
        <b style={{ fontWeight: 500 }}>{row.label}</b> · {measureOf(row.measure)?.label} by {row.axis}
      </span>
      <span className="mono" style={{ ...LINE, fontSize: 'var(--t2)' }}>
        {typed && <span style={{ ...CHIP, color: 'var(--purple)' }}>typed</span>}
        {hit && <span style={{ ...CHIP, color: STATUS[hit.status] }}>{hit.status}</span>}
        {hit && <span style={{ color: 'var(--link)', marginRight: 6 }}>{hit.id}{shift(hit)}</span>}
        {row.terms.length ? terse(row.terms, PEEK) : '…'}
      </span>
    </button>
    <button type="button" className="icon" aria-label={on ? 'Drop' : 'Add'} aria-pressed={on} disabled={on && last} onClick={add} style={{ minWidth: '1.75rem', fontFamily: 'var(--mono)' }}>
      {on ? '−' : '+'}
    </button>
  </li>
);

const PAGER = { display: 'flex', gap: 'var(--s2)', alignItems: 'center', padding: 'var(--s1) var(--s1) var(--s2)', fontSize: 'var(--t2)' };

export function Widget({ value, onChange, onReady, onExport, onStatus }) {
  const math = useUnit('math');
  const [tick, setTick] = useState(0);
  const [page, setPage] = useState(0);
  const live = useRef({ rows: [], version: 0 });
  const stage = useRef(null);
  const now = useRef(value);
  const tell = useRef(onChange);
  const field = useRef(null);
  const listed = useRef([]);
  const chosen = useRef([]);
  const rolled = useRef(value.seed);
  now.current = value;
  tell.current = onChange;
  const book = useMemo(() => math && ledger(math), [math]);
  const { dim, base, measure, axis, chart, digits, terms, seed, q, pick } = value;
  const space = useMemo(() => (book ? filter(book.keys(dim, base), measure, axis) : []), [book, dim, base, measure, axis]);
  const homes = useMemo(() => new Set(space.map((row) => row.key)), [space]);
  const named = (needle) => {
    if (!book) return [];
    const found = [book.file(needle), ...keyed(RECORDS, needle).map((record) => book.file(record.key))].filter(Boolean);
    return filter([...new Map(found.map((row) => [row.key, row])).values()], measure, axis);
  };
  const list = useMemo(() => (book ? search(space, q, { found: hit, typed: (code) => filter(book.typed(dim, base, code), measure, axis), named }) : []), [book, space, q, tick]);
  listed.current = list;
  const listedKeys = useMemo(() => new Set(list.map((row) => row.key)), [list]);
  const keys = useMemo(() => picks(pick), [pick]);
  const rows = useMemo(() => {
    if (!book) return [];
    const own = keys.map((key) => book.row(parse(key)));
    if (own.length) return own;
    return space.length ? [lead(book, space, roll(seed, space.length))] : [];
  }, [book, keys, space, seed]);
  chosen.current = rows;
  const chosenKey = rows.map((row) => row.key).join('.');
  const write = (next) => tell.current?.({ pick: next.slice(0, MOST).join('.') });
  useEffect(() => {
    if (!book || rolled.current === seed) return;
    rolled.current = seed;
    const from = listed.current.length ? listed.current : space;
    const next = from.length ? lead(book, from, roll(seed, from.length), chosen.current[0]?.key) : null;
    if (next) write([next.key]);
  }, [book, seed]);
  const typed = numbers(q);
  const asked = typed && typed.length > 1;
  const first = list[0]?.key ?? '';
  useEffect(() => {
    if (!asked || !first || chosen.current.some((row) => listed.current.includes(row))) return;
    write([first]);
  }, [q, first]);
  useEffect(() => setPage(0), [dim, base, measure, axis, q]);
  useEffect(() => {
    const at = listed.current.findIndex((row) => row.key === chosen.current[0]?.key);
    if (at >= 0) setPage(Math.floor(at / PER));
  }, [chosenKey]);
  useEffect(() => {
    rows.forEach(hit);
    live.current = { rows: rows.map((row) => ({ ...row, terms: row.terms.slice(0, terms) })), version: live.current.version + 1 };
    stage.current?.draw?.();
  }, [rows, terms, tick]);
  useEffect(() => {
    if (!book) return undefined;
    let timer = 0;
    let gone = false;
    let last = 0;
    const work = () => {
      const end = performance.now() + SLICE;
      let more = true;
      let did = false;
      while (more && performance.now() < end) {
        more = chosen.current.some((row) => book.step(row, terms, DEEP.cells)) || book.sweep(dim, base);
        did ||= more;
      }
      if (did && (!more || performance.now() - last > PULSE)) {
        last = performance.now();
        setTick((n) => n + 1);
      }
      if (more && !gone) timer = setTimeout(work, 0);
    };
    work();
    return () => {
      gone = true;
      clearTimeout(timer);
    };
  }, [book, dim, base, terms, chosenKey]);
  const choose = (row, e) => {
    const held = e?.shiftKey || e?.metaKey || e?.ctrlKey;
    if (held) return toggle(row);
    write([row.key]);
  };
  const toggle = (row) => {
    const had = chosen.current.map((one) => one.key);
    if (had.includes(row.key) && had.length === 1) return;
    write(had.includes(row.key) ? had.filter((one) => one !== row.key) : [...had, row.key]);
  };
  const browse = (by) => {
    const at = listed.current.findIndex((row) => row.key === chosen.current[0]?.key);
    const next = listed.current[Math.max(0, Math.min(listed.current.length - 1, at + by))];
    if (next) write([next.key]);
  };
  const find = () => field.current?.focus();
  const cycle = () => {
    const at = KINDS.findIndex(([slug]) => slug === now.current.chart);
    tell.current?.({ chart: KINDS[(at + 1) % KINDS.length][0] });
  };
  const plot = () => {
    const row = chosen.current[0];
    if (row) location.assign(plotted(row, now.current.terms));
  };
  const ready = (handle) => {
    stage.current = handle;
    onReady?.(handle && { ...handle, browse, find, cycle, plot });
  };
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, live, math }), [math]);
  const tap = (e) => {
    const handle = stage.current;
    if (!handle?.tap) return;
    const box = e.currentTarget.getBoundingClientRect();
    const k = e.currentTarget.width / (box.width || 1);
    const got = handle.tap((e.clientX - box.left) * k, (e.clientY - box.top) * k);
    if (got?.row > 0) {
      const had = chosen.current.map((one) => one.key);
      write([had[got.row], ...had.filter((_, i) => i !== got.row)]);
    }
  };
  const sheet = (kind) => {
    const data = chosen.current.map((row) => ({ ...row, terms: row.terms.slice(0, now.current.terms) }));
    if (data.length) text(kind === 'csv' ? csv(data) : json(data), named(now.current), kind);
  };
  const have = rows.length > 0;
  useEffect(() => onExport?.(have ? [['CSV', () => sheet('csv')], ['JSON', () => sheet('json')]] : []), [have]);
  const head = rows[0];
  const told = book && head ? facts(book, head, terms) : null;
  const toldKey = told ? `${head.key}|${told.record?.id}|${told.record?.shift}|${told.status}|${told.terms}|${told.closed}|${told.found.map(({ record }) => record.id).join()}|${terms}` : '';
  useEffect(() => {
    if (!told) return onStatus?.([]);
    onStatus?.([
      ['record', told.record ? <Record hit={told.record} /> : 'none'],
      ['status', told.status],
      ['closed form', told.closed],
      ['terms', told.terms],
      ['plot', <a href={plotted(head, terms)}>draw this row</a>],
      ['space', told.space],
      ['axis', told.axis],
      ['name', head.file],
      ['found', <Found found={told.found} />],
    ]);
  }, [toldKey]);
  const done = book ? book.progress(dim, base) : { done: 0, total: 0 };
  const picked = measureOf(measure);
  const shown = [...rows.filter((row) => !listedKeys.has(row.key)), ...list];
  const pages = Math.max(1, Math.ceil(shown.length / PER));
  const at = Math.min(page, pages - 1);
  const slice = shown.slice(at * PER, (at + 1) * PER);
  const holders = asked ? identify(RECORDS, typed).slice(0, 3) : [];
  const label = head ? `The terms of ${title(head)}, charted as ${KINDS.find(([one]) => one === chart)?.[1].toLowerCase()} beside the design` : 'Sequences';
  return (
    <>
      {book && <Scene make={scene} value={{ chart, digits }} onReady={ready} onPointerUp={tap} role="img" aria-label={label} />}
      <Bar side="left">
        <section aria-label="Search">
          <h3>Search</h3>
          <input
            ref={field}
            type="search"
            value={q}
            placeholder="8, 21, 40 or carpet or A000567 or 23"
            aria-label="Search"
            onChange={(e) => tell.current?.({ q: e.target.value })}
            onKeyDown={(e) => {
              if (e.key === 'Enter' && listed.current[0]) {
                e.preventDefault();
                write([listed.current[0].key]);
              }
              if (e.key === 'Escape') e.currentTarget.blur();
            }}
            style={{ width: '100%', background: 'var(--panel)', color: 'var(--fg)', border: 'var(--hair)', borderRadius: 'var(--r1)', padding: 'var(--s1) var(--s2)', minHeight: '2rem', font: 'var(--t3) var(--mono)' }}
          />
          {asked && (
            <p className="dim" style={{ padding: 'var(--s2) var(--s1) 0', fontSize: 'var(--t2)', lineHeight: 1.5 }}>
              <Found found={holders} />
            </p>
          )}
        </section>
        <section aria-label="Rows">
          <h3>{picked && !applies(picked, dim, base) ? `no ${picked.label.toLowerCase()} in this space` : `${list.length} rows${done.done < done.total ? ` · reading ${done.done} of ${done.total}` : ''}`}</h3>
          {pages > 1 && (
            <div style={PAGER}>
              <button type="button" disabled={at === 0} onClick={() => setPage(at - 1)}>prev</button>
              <span className="mono">{`${at * PER + 1}-${Math.min(shown.length, (at + 1) * PER)} of ${shown.length}`}</span>
              <button type="button" disabled={at >= pages - 1} onClick={() => setPage(at + 1)}>next</button>
            </div>
          )}
          <ul style={{ listStyle: 'none', padding: 0, margin: 0 }}>
            {slice.map((row) => {
              const on = rows.findIndex((one) => one.key === row.key);
              return <Row key={row.key} row={row} hit={hit(row)} on={on >= 0} tone={on >= 0 ? tone(on, rows.length) : ''} typed={listedKeys.has(row.key) && !homes.has(row.key)} last={rows.length === 1} choose={(e) => choose(row, e)} add={() => toggle(row)} />;
            })}
          </ul>
        </section>
      </Bar>
      {!book && (
        <Bar side="status">
          <span>loading</span>
        </Bar>
      )}
    </>
  );
}
