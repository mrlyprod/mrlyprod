import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { text } from '../../lib/app/export.js';
import { Bar } from '../../lib/app/frame.jsx';
import { Fold } from '../../lib/app/controls.jsx';
import { rng } from '../../lib/rand.js';
import { Scene } from '../../lib/app/page.jsx';
import { next, roll } from '../../lib/designs/engine.js';
import { Pick } from '../../lib/designs/pick.jsx';
import { KINDS } from '../../lib/sequences/chart.js';
import { DEEP, TERMS, csv, json, ledger, measureOf, parse, print } from '../../lib/sequences/engine.js';
import { facts, found, hit } from '../../lib/sequences/facts.js';
import { title } from '../../lib/sequences/paint.js';
import { Record } from '../../lib/sequences/parts.jsx';
import { PAIRED, combine, decimal, named, rule } from './engine.js';
import { filed, make } from './scene.js';
import { useUnits } from '../../lib/app/hooks.js';
import { wasm } from '../../lib/units/index.js';

const UNITS = wasm('math', 'num');

const SLICE = 8;
const PULSE = 90;

const space = (dim, base) => ({ dim, base: dim === 3 ? 2 : base });

export const key = ({ dim, base, code, measure, axis }) => {
  const one = { ...space(dim, base), code: String(code ?? '').trim(), measure, axis };
  return parse(print(one)) ? one : null;
};

export function panels(num, a, b, value) {
  const count = value.terms;
  const left = a.terms.slice(0, count);
  const out = [{ row: { ...a, terms: left }, rule: rule(num, left), note: a.error ? `a read error: ${a.error}` : '' }];
  if (Number(value.mix) !== 1) return out;
  const paired = PAIRED.includes(value.op);
  const right = paired && b ? b.terms.slice(0, count) : [];
  const made = combine(num, left, right, value.op, value.arg);
  const tag = paired ? (b ? `${b.label} ${measureOf(b.measure)?.label.toLowerCase()}` : 'no second row') : `argument ${value.arg}`;
  const row = { key: `mix-${value.op}-${value.arg}-${a.key}-${paired && b ? b.key : ''}`, title: `${named(value.op)} of ${a.label} ${measureOf(a.measure)?.label.toLowerCase()}${paired ? ` and ${tag}` : `, ${tag}`}`, terms: made.terms, start: 0, say: paired ? 'both sequences, index from 0' : 'the first sequence, index from 0', hit: found(made.terms) };
  const note = made.error ? `the mix failed: ${made.error}` : made.cut ? 'the mix stops where 128 bits do' : '';
  out.push({ row, rule: rule(num, made.terms), note, mate: paired ? b : null });
  return out;
}

export function Widget({ value, onChange, onReady, onExport, onStatus }) {
  const unit = useUnits(UNITS);
  const [tick, setTick] = useState(0);
  const live = useRef({ panels: [], version: 0 });
  const stage = useRef(null);
  const now = useRef(value);
  const tell = useRef(onChange);
  const rolled = useRef(value.seed);
  now.current = value;
  tell.current = onChange;
  const math = unit?.math;
  const book = useMemo(() => math && ledger(math), [math]);
  const { dim, base, code, measure, axis, terms, chart, digits, seed, scode, smeasure, op, arg } = value;
  const mixed = Number(value.mix) === 1;
  const paired = mixed && PAIRED.includes(op);
  const one = key({ dim, base, code, measure, axis });
  const two = paired ? key({ dim, base, code: scode, measure: smeasure, axis }) : null;
  const a = book && one ? book.row(one) : null;
  const b = book && two ? book.row(two) : null;
  useEffect(() => {
    if (dim === 3 && base !== 2) tell.current?.({ base: 2 });
  }, [dim, base]);
  useEffect(() => {
    if (!math || rolled.current === seed) return;
    rolled.current = seed;
    const { dim: d, base: q } = space(dim, base);
    tell.current?.({ code: roll(math, d, q, rng(seed >>> 0)) });
  }, [math, seed]);
  useEffect(() => {
    if (!book || !a) return undefined;
    let timer = 0;
    let gone = false;
    let last = 0;
    const work = () => {
      const end = performance.now() + SLICE;
      let more = true;
      let did = false;
      while (more && performance.now() < end) {
        more = book.step(a, terms, DEEP.cells) || Boolean(b && book.step(b, terms, DEEP.cells));
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
  }, [book, a, b, terms]);
  const shown = useMemo(() => {
    if (!unit || !a) return [];
    hit(a);
    if (b) hit(b);
    return panels(unit.num, a, b, value);
  }, [unit, a, b, terms, mixed, op, arg, tick]);
  useEffect(() => {
    live.current = { panels: shown, version: live.current.version + 1 };
    stage.current?.draw?.();
  }, [shown]);
  const codes = () => (book ? book.designs(space(now.current.dim, now.current.base).dim, space(now.current.dim, now.current.base).base).map((one) => one.code) : []);
  const handle = {
    mix: () => tell.current?.({ mix: Number(now.current.mix) === 1 ? 0 : 1 }),
    design: (by) => tell.current?.({ code: next(codes(), now.current.code, by) }),
    terms: (by) => tell.current?.({ terms: Math.max(TERMS.min, Math.min(TERMS.max, now.current.terms + by)) }),
    cycle: () => {
      const at = KINDS.findIndex(([slug]) => slug === now.current.chart);
      tell.current?.({ chart: KINDS[(at + 1) % KINDS.length][0] });
    },
  };
  const ready = (got) => {
    stage.current = got;
    onReady?.(got && { ...got, ...handle });
  };
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, live }), []);
  const tap = (e) => {
    const got = stage.current;
    if (!got?.tap) return;
    const box = e.currentTarget.getBoundingClientRect();
    const k = e.currentTarget.width / (box.width || 1);
    got.tap((e.clientX - box.left) * k, (e.clientY - box.top) * k);
  };
  const sheet = (kind) => {
    const rows = [a, b].filter(Boolean).map((row) => ({ ...row, terms: row.terms.slice(0, now.current.terms) }));
    if (rows.length) text(kind === 'csv' ? csv(rows) : json(rows), filed(now.current), kind);
  };
  const have = Boolean(a);
  useEffect(() => onExport?.(have ? [['CSV', () => sheet('csv')], ['JSON', () => sheet('json')]] : []), [have]);
  const lead = shown[0];
  const blend = shown[1];
  const told = book && a ? facts(book, a, terms) : null;
  const mate = blend?.mate ? rule(unit.num, blend.mate.terms.slice(0, terms)) : null;
  const toldKey = told ? `${a.key}|${told.record?.id}|${told.status}|${told.terms}|${lead?.rule.recurrence}|${lead?.rule.order}|${blend?.row.key}|${blend?.rule.recurrence}|${blend?.row.terms.length}|${mate?.growth}` : '';
  useEffect(() => {
    if (!told || !lead) return onStatus?.([]);
    const rows = [
      ['order', lead.rule.order ?? 'none fits'],
      ['growth', decimal(lead.rule.growth)],
      ['closed form', told.closed],
      ['record', told.record ? <Record hit={told.record} /> : 'none'],
      ['status', told.status],
      ['terms', told.terms],
      ['rule', lead.rule.recurrence || 'no linear rule fits these terms'],
      ['characteristic', lead.rule.polynomial || 'none'],
      ['root', decimal(lead.rule.root)],
      ['exponent', decimal(lead.rule.exponent)],
      ['read from', lead.rule.from],
    ];
    if (blend) {
      rows.push(['mix order', blend.rule.order ?? 'none fits'], ['mix growth', decimal(blend.rule.growth)], ['mix rule', blend.rule.recurrence || 'no linear rule fits the mix']);
      if (blend.mate) rows.push(['second', title(blend.mate)], ['its growth', decimal(mate?.growth)]);
      rows.push(['argument', PAIRED.includes(op) ? 'unused' : String(arg)]);
    }
    rows.push(['ledger', <a href={`../sequences/?pick=${a.key}`}>open in sequences</a>]);
    onStatus?.(rows);
  }, [toldKey]);
  const label = a ? `The terms of ${title(a)} as ${KINDS.find(([one]) => one === chart)?.[1].toLowerCase()}, with the recurrence they satisfy${mixed ? ', and the mix beside it' : ''}` : 'Plot';
  const pick = (next) => tell.current?.({ ...space(next.dim, next.base), code: next.code });
  return (
    <>
      {unit && <Scene make={scene} value={{ chart, digits }} onReady={ready} onPointerUp={tap} role="img" aria-label={label} />}
      <Bar side="right">
        <Fold name="Design" live={a ? a.label : ''}>
          {math ? <Pick value={{ dim, base: space(dim, base).base, code }} onChange={pick} math={math} /> : <span>loading</span>}
        </Fold>
      </Bar>
      {!one && book && (
        <Bar side="status">
          <span>{`no row for code ${code} in dimension ${dim}`}</span>
        </Bar>
      )}
    </>
  );
}
