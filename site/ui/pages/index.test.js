import { expect, test } from 'bun:test';
import { readdirSync } from 'node:fs';
import { state } from '../../scripts/site.ts';
import { LAYOUT } from './index.js';

test('every kind in the rows has a page and a layout, and every layout a page', () => {
  const kinds = [...new Set(state().rows.map((row) => row.kind))].sort();
  const files = readdirSync(import.meta.dir).filter((name) => name.endsWith('.jsx')).map((name) => name.slice(0, -4)).sort();
  expect([kinds.filter((kind) => !files.includes(kind) || !LAYOUT[kind]), Object.keys(LAYOUT).sort()]).toEqual([[], files]);
});
