import { expect, test } from 'bun:test';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ICONS } from './icons.js';

const site = resolve(import.meta.dir, '..');
const NAMES = JSON.parse(readFileSync(resolve(site, 'icons.json'), 'utf8'));

test('icons.js holds one path for each name icons.json lists', () => {
  expect(Object.keys(ICONS)).toEqual(NAMES);
  for (const name of NAMES) expect(ICONS[name]).toMatch(/^m/i);
});

test('every icon the code draws is listed in icons.json', () => {
  const code = ['lib', 'apps'].flatMap((top) => [...new Bun.Glob(`${top}/**/*.jsx`).scanSync(site)]);
  const used = code.flatMap((file) => [...readFileSync(resolve(site, file), 'utf8').matchAll(/<Icon name="([a-z_]+)"/g)].map((m) => m[1]));
  expect(used.filter((name) => !NAMES.includes(name))).toEqual([]);
});
