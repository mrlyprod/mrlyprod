import { expect, test } from 'bun:test';
import { law } from './lock.ts';

const pin = { key: 'p', files: ['p.png'] };

test('a lock that names every figure once and matches the press is clean', () => {
  expect(law(['a', 'b'], { a: 'ka', b: 'kb', lab: pin }, { a: 'ka', b: 'kb' })).toEqual([]);
  expect(law(['a'], { a: 'ka' }, null)).toEqual([]);
});

test('a figure .ts with no pressed row is red, a pinned row does not stand for it', () => {
  expect(law(['a', 'b'], { a: 'ka' }, null)).toEqual(['figures/b.ts has no pressed row in site/figures.lock']);
  expect(law(['a'], { a: pin }, null)).toEqual(['figures/a.ts has no pressed row in site/figures.lock']);
});

test('a pressed row with no figure .ts is red, a pinned row is not', () => {
  expect(law(['a'], { a: 'ka', gone: 'kg', lab: pin }, null)).toEqual(['gone is a pressed row of site/figures.lock with no figures/gone.ts']);
});

test('a desk key that differs from the committed row is one red line, and a missing desk key is not', () => {
  const bad = law(['a', 'b', 'c', 'd'], { a: 'k1', b: 'k2', c: 'k3', d: 'k4' }, { a: 'x', b: 'x', c: 'x', d: 'k4', e: 'x' });
  expect(bad).toEqual(['the lock is behind the press on 3 figures (a, b, c); run the figures console']);
  expect(law(['a'], { a: 'k1' }, {})).toEqual([]);
});
