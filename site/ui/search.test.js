import { expect, test } from 'bun:test';
import { meets, terms } from './search.js';

test('a row is found only when every word of the query is in it, in any case and any order', () => {
  const row = 'The Farey sequence /research/wiki/farey-sequence/';
  const rows = [
    ['farey', true],
    ['  WIKI   Farey ', true],
    ['sequence the', true],
    ['farey demos', false],
    ['fareys', false],
  ];
  for (const [query, want] of rows) expect([query, meets(terms(query), row)]).toEqual([query, want]);
  expect(terms('   ')).toEqual([]);
});
