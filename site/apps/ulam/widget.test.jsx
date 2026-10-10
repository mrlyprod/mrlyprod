import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { Frame } from '../../lib/frame.jsx';
import { defaults } from '../../lib/knobs.js';
import { study } from './engine.js';
import { SPEC } from './scene.js';
import { Widget, counts, pin, streak } from './widget.jsx';

num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { ...defaults(SPEC), seed: 1 };

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Ulam"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(num)).toContain('<canvas');
});

test('the streak reads the opening run of primes and the composite that ends it', () => {
  expect(streak({ count: 0 })).toBe('off the sheet');
  expect(streak({ count: 5, streak: 5 })).toBe('all 5 prime');
  expect(streak({ count: 101, streak: 21, next: { n: 1763, factors: [[41, 1], [43, 1]] } })).toBe('21, then 1763 = 41 · 43');
});

test('the counts are one list: the rolled lattice only while random, the sheet, the lit cells only when the mark is not the primes, and the line only with a line', () => {
  const read = (over) => counts(study(num, { ...value, fit: 1, rings: 20, roll: 0, ...over }, 800, 600, 2).facts).map(([name]) => name);
  expect(read({ a: 0 })).toEqual(['Rings', 'Numbers', 'Primes', 'Density']);
  expect(read({ a: 4, mark: 'twin' })).toEqual(['Rings', 'Numbers', 'Primes', 'Density', 'Lit', 'Line', 'Landings', 'Hits', 'Share', 'Streak']);
  const rolled = counts(study(num, { ...value, fit: 1, rings: 20, roll: 1, lattice: 'hex', a: 3, b: 1, c: 7 }, 800, 600, 2).facts);
  expect([rolled[0], rolled.find(([name]) => name === 'Line')]).toEqual([['Lattice', 'Hex'], ['Line', '3k² + 1k + 7']]);
  expect(counts(study(num, { ...value, fit: 1, rings: 20, roll: 0 }, 800, 600, 2).facts).find(([name]) => name === 'Line')[1]).toBe('4k² - 2k + 41');
});

test('a key on a random sheet pins the rolled lattice and line as a named one', () => {
  const facts = study(num, { ...value, fit: 1, rings: 20, roll: 1, lattice: 'hex', a: 3, b: 1, c: 7 }, 800, 600, 2).facts;
  expect(pin(facts, { c: 11 })).toEqual({ roll: 0, lattice: 'hex', a: 3, b: 1, c: 11 });
  expect(pin(facts, { lattice: 'square' })).toEqual({ roll: 0, lattice: 'square', a: 3, b: 1, c: 7 });
});
