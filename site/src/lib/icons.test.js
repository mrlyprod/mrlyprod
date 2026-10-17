import { expect, test } from 'bun:test';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import SITE from '../../site.json';
import { ICONS } from './icons.gen.js';

const site = resolve(import.meta.dir, '..', '..');
const NAMES = SITE.icons.names;

test('icons.gen.js holds one path for each name site.json lists', () => {
  expect(Object.keys(ICONS)).toEqual(NAMES);
  for (const name of NAMES) expect(ICONS[name]).toMatch(/^m/i);
});

test('every icon the code draws is listed in the icons block of site.json', () => {
  const code = ['src/lib', 'src/pages', 'src/apps'].flatMap((top) => [...new Bun.Glob(`${top}/**/*.jsx`).scanSync(site)]);
  const used = code.flatMap((file) => [...readFileSync(resolve(site, file), 'utf8').matchAll(/<Icon name="([a-z_]+)"/g)].map((m) => m[1]));
  expect(used.filter((name) => !NAMES.includes(name))).toEqual([]);
});
