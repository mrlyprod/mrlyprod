import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { rng } from '../../lib/scene.js';
import { Scene } from '../../lib/scene.jsx';
import { ledger } from '../sequences/engine.js';
import { CARDS, build, shown } from './cards.js';
import { make } from './scene.js';

const STILL = {};

export function rolled(seed) {
  const rand = rng(seed >>> 0);
  const card = Math.floor(rand() * CARDS.length);
  const one = CARDS[card];
  return { card, step: one.min + Math.floor(rand() * (one.max - one.min + 1)) };
}

export const turned = (card, by) => {
  const next = (card + by + CARDS.length) % CARDS.length;
  return { card: next, step: CARDS[next].first };
};

export const walked = (card, step, by) => {
  const one = CARDS[card];
  const next = step + by;
  return { step: next > one.max ? one.min : next < one.min ? one.max : next };
};

export function Widget({ value, onChange, onReady, onStatus }) {
  const [units, setUnits] = useState(null);
  const live = useRef({ made: null, version: 0 });
  const stage = useRef(null);
  const now = useRef(value);
  const tell = useRef(onChange);
  const seeded = useRef(value.seed);
  now.current = value;
  tell.current = onChange;
  useEffect(() => {
    let on = true;
    import('./unit.js')
      .then(async ({ math, num, ready }) => {
        await ready;
        if (on) setUnits({ math, num });
      })
      .catch(console.error);
    return () => {
      on = false;
    };
  }, []);
  const book = useMemo(() => units && ledger(units.math), [units]);
  const { card, step, seed } = value;
  useEffect(() => {
    if (seeded.current === seed) return;
    seeded.current = seed;
    tell.current?.(rolled(seed));
  }, [seed]);
  const made = useMemo(() => (units && book ? build({ ...units, book }, card, step) : null), [units, book, card, step]);
  const said = shown(card, step);
  useEffect(() => {
    live.current = { made, card, shown: `${CARDS[card].label.toLowerCase()} ${said}`, version: live.current.version + 1 };
    stage.current?.draw?.();
  }, [made]);
  const link = made?.lines.find((line) => line.link);
  const toldKey = made ? `${card}|${step}|${made.lines.map((line) => line.terms.join()).join('|')}` : '';
  useEffect(() => {
    if (!made) return onStatus?.([]);
    const rows = [[CARDS[card].label.toLowerCase(), said]];
    made.lines.forEach((line) => rows.push([line.label, `${line.terms.join(', ')}${line.capped ? ', to the budget' : ''}`]));
    if (link) rows.push(['ledger', <a href={link.link}>open in sequences</a>]);
    onStatus?.(rows);
  }, [toldKey]);
  const handle = {
    turn: (by) => tell.current?.(turned(now.current.card, by)),
    walk: (by) => tell.current?.(walked(now.current.card, now.current.step, by)),
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
    if (got.tap((e.clientX - box.left) * k, (e.clientY - box.top) * k)?.picture) handle.walk(1);
  };
  const one = CARDS[card];
  return units ? <Scene make={scene} value={STILL} onReady={ready} onPointerUp={tap} role="img" aria-label={`${card + 1}, ${one.title}: ${one.say}`} /> : null;
}
