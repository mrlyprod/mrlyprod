import { expect, test } from 'bun:test';
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { own, state } from '../scripts/site.ts';

const APPS = join(import.meta.dir, '..', 'apps');

test('every row is an app or prose, an app row with its index.jsx, a prose row with a source, and each kind has its shell', () => {
  const { rows } = state();
  expect([...new Set(rows.map((row) => row.kind))].sort()).toEqual(['app', 'prose']);
  expect(['app', 'prose'].filter((kind) => !existsSync(join(import.meta.dir, `${kind}.jsx`)))).toEqual([]);
  expect(rows.filter((row) => (row.kind === 'app' ? !existsSync(join(APPS, row.meta.id, 'index.jsx')) : !row.source)).map((row) => row.route)).toEqual([]);
});

test("an app page draws as an island under its still or as a page of its own, and unmounts either way", async () => {
  const { rows } = state();
  const bad = [];
  const pages = [];
  for (const row of rows.filter((one) => one.kind === 'app')) {
    const src = await Bun.file(join(APPS, row.meta.id, 'index.jsx')).text();
    const page = /^export (async )?function render\b|^export const \{[^}]*\brender\b/m.test(src);
    if (page) pages.push(row.meta.id);
    const parts = page ? ['render', 'unmount', 'layout'] : ['mount', 'unmount'];
    if (!parts.every((name) => new RegExp(`^export (async )?(function|const) ${name}\\b|^export const \\{[^}]*\\b${name}\\b`, 'm').test(src))) bad.push(row.meta.id);
  }
  expect(bad).toEqual([]);
  expect(own(state().apps).sort()).toEqual(pages.sort());
});
