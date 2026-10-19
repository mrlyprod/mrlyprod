import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { gallery } from './engine.js';
import { choose, star } from './store.js';
import { Browser, named, reach, search, shows, span, step } from './browse.jsx';

const FREE = { dim: 'dim', base: 'base', code: 'code' };

const memory = () => {
  const box = new Map();
  return { getItem: (key) => (box.has(key) ? box.get(key) : null), setItem: (key, value) => box.set(key, String(value)), removeItem: (key) => box.delete(key) };
};

const listed = (dim, base, show) => gallery(math, dim, base, show).map((code) => ({ dim, base, code }));

const found = (dim, base, show, query) => search(math, { dim, base }, listed(dim, base, show), query).map((one) => one.code);

const groups = (html, role) => [...html.matchAll(new RegExp(`role="${role}" aria-label="([^"]*)"`, 'g'))].map((one) => one[1]);

const cells = (html) => [...html.matchAll(/aria-pressed="(true|false)" aria-label="([^"]*)"/g)].map((one) => (one[1] === 'true' ? `[${one[2]}]` : one[2]));

test('a design reads as its name and code, an unnamed one as code and number, and as its code alone while math loads', () => {
  expect([named(math, { dim: 2, base: 2, code: '7' }), named(math, { dim: 2, base: 2, code: '11' }), named(math, { dim: 2, base: 2, code: '99' }), named(null, { dim: 2, base: 2, code: '7' })]).toEqual(['carpet 7', 'code 11', 'code 99', '7']);
});

test('a space offers its classes, names and codes only where each can be listed', () => {
  const keys = (dim, base) => shows(math, dim, base).map(([one]) => one);
  expect([keys(2, 2), keys(2, 4), keys(3, 3)]).toEqual([['classes', 'named', 'all'], ['classes', 'named'], ['named']]);
});

test('search matches a name inside, a code from its start, a base 2 normal form inside, and puts a typed code of the space first', () => {
  expect([found(2, 2, 'all', 'CAR'), found(2, 2, 'all', 'xy'), found(2, 3, 'classes', '1'), found(3, 3, 'named', '5'), found(2, 2, 'classes', ' 08 '), found(2, 2, 'classes', '99')]).toEqual([['7'], ['1', '2', '4', '7', '8', '11', '13', '14'], ['1', '10', '11', '14', '15', '118', '119', '127'], ['5'], ['8'], []]);
});

test('arrows walk a grid of ten in rows of four and stop at its edges', () => {
  expect(['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Home', 'End', 'a'].map((key) => step(5, key, 4, 10))).toEqual([4, 6, 1, 9, 0, 9, -1]);
  expect([step(7, 'ArrowDown', 4, 10), step(1, 'ArrowUp', 4, 10), step(9, 'ArrowRight', 4, 10)]).toEqual([7, 1, 9]);
});

test('a window of 820 rows at 100px a row draws the rows in sight and two more each side, never past either end', () => {
  expect([span(0, 450, 100, 820), span(1050, 450, 100, 820), span(81500, 450, 100, 820), span(-300, 450, 100, 820), span(0, 450, 100, 3)]).toEqual([[0, 7], [8, 17], [813, 820], [0, 4], [0, 3]]);
});

test('a row out of sight scrolls just into view with room to spare, up to its top or down to its bottom less the gap', () => {
  expect([reach(3, 100, 4, 1000, 450), reach(10, 100, 4, 1000, 450), reach(14, 100, 4, 1000, 450), reach(15, 100, 4, 1000, 450), reach(3, 100, 4, 1000, 450, 12), reach(11, 100, 4, 1000, 450, 12), reach(15, 100, 4, 1000, 450, 12)]).toEqual([-700, 0, 46, 146, -712, 0, 158]);
});

test('the browser offers a segment only for what the slot leaves free, lists the classes of the space and previews the current design', () => {
  const html = (slot, value) => renderToStaticMarkup(<Browser slot={slot} value={value} math={math} onPick={() => {}} />);
  const free = html(FREE, { dim: 2, base: 2, code: '7' });
  const fixed = html({ dim: 2, base: 'base', code: 'code', bases: [2] }, { base: 2, code: '' });
  expect([groups(free, 'radiogroup'), groups(fixed, 'radiogroup')]).toEqual([['Dim', 'Base', 'Show'], ['Show']]);
  expect(cells(free)).toEqual(['code 0', 'dust 1', 'htree 3', 'star 6', '[carpet 7]', 'code 15']);
  expect([...free.matchAll(/<dd>([^<]*)<\/dd>/g)].map((one) => one[1])).toEqual(['7', '2D, base 2', '7', '4', '1+xy']);
});

test('a long list shows 120 in the compact browser with a pointer to Browse all, and the full one opens on its first 120 with no More button', () => {
  const html = (compact) => renderToStaticMarkup(<Browser compact={compact} slot={{ dim: 2, base: 'base', code: 'code', bases: [2, 3, 4] }} value={{ base: 4, code: '24415' }} math={math} onPick={() => {}} />);
  const [small, full] = [html(true), html(false)];
  expect([cells(small).length, small.includes('685 more in Browse all'), small.includes('class="view"')]).toEqual([120, true, false]);
  expect([cells(full).length, full.includes('class="more"'), full.includes('data-at="119"')]).toEqual([120, false, true]);
});

test('favorites and recent designs show only those that fit the slot', () => {
  globalThis.localStorage = memory();
  try {
    star({ dim: 3, base: 2, code: '23' });
    star({ dim: 2, base: 5, code: '7' });
    choose({ dim: 2, base: 2, code: '7' });
    choose({ dim: 2, base: 3, code: '495' });
    const html = renderToStaticMarkup(<Browser compact slot={FREE} value={{ dim: 2, base: 2, code: '6' }} math={math} onPick={() => {}} />);
    expect([groups(html, 'group'), cells(html).slice(0, 3)]).toEqual([['Favorites', 'Recent', '2D base 2'], ['carpet 23', 'carpet 495', 'carpet 7']]);
  } finally {
    delete globalThis.localStorage;
  }
});
