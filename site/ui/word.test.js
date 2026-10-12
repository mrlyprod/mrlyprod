import { expect, test } from 'bun:test';
import { state } from '../scripts/site.ts';
import { word } from './word.js';

const { site, rows } = state();

const list = site.doors;

test('every app with a folder route is a door that names itself by its title, and home names nothing', () => {
  const apps = rows.filter((row) => row.meta.id && row.route !== '/' && row.route.endsWith('/'));
  expect(list.map(([href]) => href).sort()).toEqual(apps.map((row) => row.route).sort());
  expect(apps.map((row) => word(row.route, list))).toEqual(apps.map((row) => row.title.toLowerCase()));
  expect(word('/', list)).toBe('');
});

test('a deep route takes its deepest door, else its first segment, and a file names nothing', () => {
  const rows = [
    ['/research/beneath/', 'research'],
    ['/research/', 'research'],
    ['/git/site/kit/push.ts', 'code'],
    ['/about/', 'about'],
    ['/settings/', 'settings'],
    ['/404.html', ''],
  ];
  expect(rows.map(([route]) => word(route, list))).toEqual(rows.map(([, name]) => name));
  expect(word('/a/b/x/', [['/a/b/', 'b'], ['/a/', 'a']])).toBe('b');
  expect(word('/raw/x/', list)).toBe('raw');
});
