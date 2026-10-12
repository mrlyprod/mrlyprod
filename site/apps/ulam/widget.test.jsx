import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { Frame } from '../../lib/frame.jsx';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { read, shuffle, study } from './engine.js';
import { SPEC } from './scene.js';
import { Widget, readouts, rolled, streak } from './widget.jsx';

num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { ...defaults(SPEC), seed: 1 };

const facts = (over, pick = null) => {
  const one = { ...value, ...over };
  return { ...read(num, study(num, one, 800, 600), one), pick };
};

test('the widget mounts its stage, named for a reader, once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Ulam"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
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

test('a new seed rolls the sheet and its quadratic into the knobs, and the seed the page opened on rolls nothing', () => {
  expect([rolled(num, 7, 7), rolled(null, 8, 7)]).toEqual([null, null]);
  expect(rolled(num, 8, 7)).toEqual(shuffle(num, rng(8)));
});
