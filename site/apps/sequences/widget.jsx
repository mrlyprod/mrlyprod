import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { rng } from '../../lib/scene.js';
import { Scene } from '../../lib/scene.jsx';
import { KINDS } from './chart.js';
import { DEEP, MEASURES, MOST, TABLE, applies, badge, csv, filter, identify, json, ledger, lead, numbers, parse, picks, search } from './engine.js';
import RECORDS from './records.json' with { type: 'json' };
import { glyph, make } from './scene.js';

const SLICE = 8;
const PULSE = 90;
const PEEK = 6;
const SIDE = 3;
const THUMB = 1.25;

const measureOf = (slug) => MEASURES.find((one) => one.slug === slug)?.label ?? slug;

const roll = (seed, n) => Math.floor(rng(seed >>> 0)() * n);

const terse = (terms, count) => terms.slice(0, count).join(', ') + (terms.length > count ? ', …' : '');

const tone = (i, m) => `color-mix(in srgb, var(--accent) ${Math.round(100 - (m > 1 ? (62 * i) / (m - 1) : 0))}%, var(--art))`;

function hit(row) {
  const n = Math.min(row.terms.length, TABLE.terms);
  if (row.hitAt !== n) {
    row.hitAt = n;
    row.hit = badge(RECORDS, row, row.terms.slice(0, n));
  }
  return row.hit;
}

const ROW = { display: 'block', width: '100%', minHeight: 0, padding: '6px 8px', background: 'none', border: 0, borderLeft: '3px solid transparent', borderRadius: 0, textAlign: 'left', lineHeight: 1.4 };

const LINE = { display: 'block', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' };

function Thumb({ cells }) {
  const shape = glyph(cells);
  if (!shape) return null;
  return (
    <svg viewBox={`0 0 ${shape.width} ${SIDE}`} height={`${THUMB}em`} width={`${(THUMB * shape.width) / SIDE}em`} fill="currentColor" aria-hidden="true" style={{ flex: 'none', alignSelf: 'center', marginLeft: 6 }}>
      {cells.map((v, i) => <rect key={i} x={Math.floor(i / (SIDE * SIDE)) * (SIDE + 1) + (i % SIDE) + 0.08} y={(Math.floor(i / SIDE) % SIDE) + 0.08} width="0.84" height="0.84" opacity={v ? 1 : 0.15} />)}
    </svg>
  );
}

const Row = ({ row, hit, on, tone, choose, add }) => (
  <li style={{ display: 'flex', alignItems: 'stretch', color: on ? 'var(--fg)' : 'var(--dim)' }}>
    <Thumb cells={row.cells} />
    <button type="button" aria-pressed={on} onClick={choose} style={{ ...ROW, flex: 1, minWidth: 0, borderLeftColor: on ? tone : 'transparent', color: 'inherit' }}>
      <span style={LINE}>
        <b style={{ fontWeight: 500 }}>{row.label}</b> · {measureOf(row.measure)} by {row.axis}
      </span>
      <span className="mono" style={{ ...LINE, fontSize: 'var(--t2)' }}>
        {hit && <span style={{ color: 'var(--link)', marginRight: 6 }}>{hit.id}</span>}
        {row.terms.length ? terse(row.terms, PEEK) : '…'}
      </span>
    </button>
    <button type="button" className="icon" aria-label={on ? 'Drop' : 'Add'} aria-pressed={on} onClick={add} style={{ minWidth: '1.75rem', fontFamily: 'var(--mono)' }}>
      {on ? '−' : '+'}
    </button>
  </li>
);

export function Widget({ value, onChange, onReady, onExport }) {
  const [math, setMath] = useState(null);
  const [tick, setTick] = useState(0);
  const live = useRef({ rows: [], version: 0 });
  const stage = useRef(null);
  const now = useRef(value);
  const tell = useRef(onChange);
  const field = useRef(null);
  const listed = useRef([]);
  const chosen = useRef([]);
  now.current = value;
  tell.current = onChange;
  useEffect(() => {
    let on = true;
    import('./unit.js')
      .then(async ({ math: unit, ready }) => {
        await ready;
        if (on) setMath(unit);
      })
      .catch(console.error);
    return () => {
      on = false;
    };
  }, []);
  const book = useMemo(() => math && ledger(math), [math]);
  const { dim, base, measure, axis, chart, digits, terms, seed, q, pick } = value;
  const list = useMemo(() => (book ? search(filter(book.keys(dim, base), measure, axis), q, hit, (code) => filter(book.typed(dim, base, code), measure, axis)) : []), [book, dim, base, measure, axis, q, tick]);
  listed.current = list;
  const keys = useMemo(() => picks(pick), [pick]);
  const rows = useMemo(() => {
    if (!book) return [];
    const own = keys.map((key) => book.row(parse(key)));
    if (own.length) return own;
    return list.length ? [lead(book, list, roll(seed, list.length))] : [];
  }, [book, keys, list, seed]);
  chosen.current = rows;
  const chosenKey = rows.map((row) => row.key).join('.');
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
  const write = (next) => tell.current?.({ pick: next.slice(0, MOST).join('.') });
  const choose = (row, e) => {
    const held = e?.shiftKey || e?.metaKey || e?.ctrlKey;
    if (held) return toggle(row);
    write([row.key]);
  };
  const toggle = (row) => {
    const had = picks(now.current.pick);
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
  const ready = (handle) => {
    stage.current = handle;
    onReady?.(handle && { ...handle, browse, find, cycle });
  };
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, live }), []);
  const sheet = (kind) => {
    const { terms: count, seed: at } = now.current;
    const data = chosen.current.map((row) => ({ ...row, terms: row.terms.slice(0, count) }));
    if (data.length) text(kind === 'csv' ? csv(data) : json(data), `sequences-${at}`, kind);
  };
  const have = rows.length > 0;
  useEffect(() => onExport?.(have ? [['CSV', () => sheet('csv')], ['JSON', () => sheet('json')]] : []), [have]);
  const typed = numbers(q);
  const named = typed && typed.length > 1 ? identify(RECORDS, typed).slice(0, 3) : [];
  const first = rows[0];
  const done = book ? book.progress(dim, base) : { done: 0, total: 0 };
  const picked = MEASURES.find((one) => one.slug === measure);
  const shown = [...rows.filter((row) => !list.includes(row)), ...list];
  const key = { seed, chart, digits, terms, pick: chosenKey };
  return (
    <>
      {book && <Scene make={scene} value={key} onReady={ready} />}
      <Bar side="left">
        <section aria-label="Search">
          <h3>Search</h3>
          <input ref={field} type="search" value={q} placeholder="8, 21, 40 or carpet or A000567" aria-label="Search" onChange={(e) => tell.current?.({ q: e.target.value })} style={{ width: '100%', background: 'var(--panel)', color: 'var(--fg)', border: 'var(--hair)', borderRadius: 'var(--r1)', padding: 'var(--s1) var(--s2)', minHeight: '2rem', font: 'var(--t3) var(--mono)' }} />
          {typed && typed.length > 1 && (
            <p className="dim" style={{ padding: 'var(--s2) var(--s1) 0', fontSize: 'var(--t2)', lineHeight: 1.5 }}>
              {named.length
                ? named.map(({ record, at }) => (
                    <span key={record.id} style={{ display: 'block' }}>
                      <a href={`https://oeis.org/${record.id}`} target="_blank" rel="noopener">{record.id}</a> at {at}: {record.name}
                    </span>
                  ))
                : 'no record holds these terms'}
            </p>
          )}
        </section>
        <section aria-label="Rows">
          <h3>
            {picked && !applies(picked, dim, base) ? `no ${picked.label.toLowerCase()} in this space` : `${list.length} rows${done.done < done.total ? ` · reading ${done.done} of ${done.total}` : ''}`}
          </h3>
          <ul style={{ listStyle: 'none', padding: 0, margin: 0 }}>
            {shown.map((row) => {
              const at = rows.findIndex((one) => one.key === row.key);
              return <Row key={row.key} row={row} hit={hit(row)} on={at >= 0} tone={at >= 0 ? tone(at, rows.length) : ''} choose={(e) => choose(row, e)} add={() => toggle(row)} />;
            })}
          </ul>
        </section>
      </Bar>
      <Bar side="status">
        {!book && <span>loading</span>}
        {first && <span>{rows.length > 1 ? `${rows.length} rows` : `${first.label} · ${measureOf(first.measure)} by ${first.axis}`}</span>}
        {first?.hit && (
          <span>
            <a href={`https://oeis.org/${first.hit.id}`} target="_blank" rel="noopener">{first.hit.id}</a> {first.hit.status.toLowerCase()}
          </span>
        )}
        {first && <span className="mono">{first.terms.slice(0, terms).join(', ')}{book?.capped(first, terms, DEEP.cells) ? ' · to the budget' : ''}</span>}
      </Bar>
    </>
  );
}
