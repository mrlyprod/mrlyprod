import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as num from '@mrlyprod/mrly/num';
import { Frame } from '../../lib/app/frame.jsx';
import { defaults, tidy } from '../../lib/app/spec.js';
import { POLYS, SPIRALS, holds, read, study } from './engine.js';
import { SPEC, sketch } from './scene.js';
import { Widget, readouts, streak } from './widget.jsx';

const value = { ...defaults(SPEC), seed: 1 };

const facts = (over, pick = null) => {
  const one = { ...value, ...over };
  return { ...read(num, study(num, one, 800, 600), one), pick };
};

test('the stage is named for a reader', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Ulam"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(num)).toContain('<canvas aria-label="The sheet');
});

test('the streak reads the opening run of primes and the composite that ends it', () => {
  expect(streak({ count: 0 })).toBe('off the sheet');
  expect(streak({ count: 5, streak: 5 })).toBe('all 5 prime');
  expect(streak({ count: 101, streak: 21, next: { n: 1763, factors: [[41, 1], [43, 1]] } })).toBe('21, then 1763 = 41 · 43');
});

test('the readouts lead with the line, then the tapped cell, the first values and the counts, the lit count only off the primes', () => {
  const rows = readouts(facts({}));
  expect(rows.slice(0, 2)).toEqual([['Line', '4k² - 2k + 41'], ['Values', '41 43 53 71 97 131 173 223 …']]);
  expect(rows.map(([name]) => name)).toEqual(['Line', 'Values', 'Streak', 'Hits', 'Share', 'Numbers', 'Primes', 'Density']);
  const tapped = readouts(facts({ mark: 'twin' }, { n: 1763, ring: 21, x: 21, y: -21, prime: false, factors: [[41, 1], [43, 1]] }));
  expect(tapped.slice(0, 3)).toEqual([['Line', '4k² - 2k + 41'], ['Cell', '1763 on ring 21 at 21, -21'], ['Verdict', 'composite, 41 · 43']]);
  expect(tapped.at(-1)[0]).toBe('Lit');
  expect(readouts(facts({ a: 0 }, { off: true })).map(([name, said]) => (name === 'Cell' ? said : name))).toEqual(['off the sheet', 'Numbers', 'Primes', 'Density']);
  expect(readouts(facts({}, { n: 1, ring: 0, x: 0, y: 0, prime: false, factors: [] }))[2]).toEqual(['Verdict', 'one, neither prime nor composite']);
});

test('every library entry stands for knobs the sheet accepts, a spiral with no line and a polynomial prime for its opening values', () => {
  for (const { set } of [...SPIRALS, ...POLYS]) expect(tidy(SPEC, { ...value, ...set })).toMatchObject(set);
  expect(SPIRALS.every(({ set }) => set.a === 0)).toBe(true);
  for (const { set, note } of POLYS) {
    const line = read(num, study(num, { ...value, ...set }, 800, 600), { ...value, ...set }).line;
    expect(note).toBe(`prime for its first ${line.streak} values`);
  }
  expect([holds(value, POLYS[0].set), holds(value, SPIRALS[0].set)]).toEqual([true, false]);
});

test('a library thumb lays a cell for every number and the line over them, a spiral without a line none', () => {
  const cells = (set) => {
    let n = 0;
    const pen = new Proxy({}, { get: (_, key) => () => key === 'rect' && n++, set: () => true });
    sketch(num, set, pen, 112, 112, 1, { paper: '#000000', accent: '#ffd100' });
    return n;
  };
  const line = POLYS[0].set;
  expect(cells(SPIRALS[0].set)).toBe(53 * 53);
  expect(cells(line)).toBe(53 * 53 + study(num, tidy(SPEC, { ...line, side: 53 }), 112, 112).line.values.length);
});
