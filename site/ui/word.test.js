import { expect, test } from 'bun:test';
import SITE from '../site.json';
import { doors, word } from './word.js';

const list = doors(SITE.tree);

test('every menu door names itself and home names nothing', () => {
  const rows = list.map(([href, name]) => [word(href, list), name]);
  expect(rows.every(([got, want]) => got === want)).toBe(true);
  expect(list.length).toBeGreaterThan(5);
  expect(word('/', list)).toBe('');
});

test('a deep route takes its deepest door, else its first segment, and a file names nothing', () => {
  const rows = [
    ['/research/wiki/rep-tiles/', 'wiki'],
    ['/research/claims/beneath/', 'claims'],
    ['/research/', 'research'],
    ['/git/site/kit/ssg/build.ts', 'code'],
    ['/demos/sponge/', 'demos'],
    ['/about/', 'about'],
    ['/settings/', 'settings'],
    ['/404.html', ''],
  ];
  expect(rows.map(([route]) => word(route, list))).toEqual(rows.map(([, name]) => name));
});
