import { expect, test } from 'bun:test';
import { find, meets, terms } from './search.js';

test('a row is found only when every word of the query is in its title, route or lead, in any case and any order', () => {
  const row = 'Launching the site /blog/launching-mrlyprod-org/ One address for the demos';
  const rows = [
    ['launching', true],
    ['  BLOG   Launching ', true],
    ['address the', true],
    ['launching wiki', false],
    ['launchings', false],
  ];
  for (const [query, want] of rows) expect([query, meets(terms(query), row)]).toEqual([query, want]);
  expect(terms('   ')).toEqual([]);
});

test('search returns each route once, rows first, at most 24', () => {
  const list = [...Array.from({ length: 30 }, (_, n) => ({ title: `Page ${n}`, route: `/p${n}/`, lead: '' })), { title: 'Page 0 again', route: '/p0/', lead: '' }];
  const found = find(terms('page'), list);
  expect([found.length, new Set(found.map((one) => one.route)).size, found[0].route]).toEqual([24, 24, '/p0/']);
});
