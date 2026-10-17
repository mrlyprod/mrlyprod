import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import * as num from '@mrlyprod/mrly/num';
import { values } from '../../lib/app/page.jsx';
import { DEEP, ledger } from '../../lib/sequences/engine.js';
import { SPEC } from './scene.js';
import { Widget, key, panels } from './widget.jsx';

const VALUE = values({ seed: 0, dim: 2, base: 2, code: '7' }, SPEC);

test('the page opens on the carpet perimeter by level, and before the unit loads renders nothing on the server', () => {
  expect(VALUE).toMatchObject({ dim: 2, base: 2, code: '7', measure: 'surface', axis: 'level', terms: 12, mix: 0 });
  expect(renderToStaticMarkup(<Widget value={VALUE} onChange={() => {}} onReady={() => {}} />)).toBe('');
});

test('a key needs a code the space holds, and the cube reads base 2', () => {
  expect(key({ dim: 3, base: 3, code: '23', measure: 'fills', axis: 'level' })).toEqual({ dim: 3, base: 2, code: '23', measure: 'fills', axis: 'level' });
  expect(key({ dim: 2, base: 2, code: '', measure: 'fills', axis: 'level' })).toBe(null);
  expect(key({ dim: 2, base: 2, code: '7', measure: 'faces', axis: 'level' })).toBe(null);
});

test('the panels hold the row and its rule, then with the mix the blend of the pair under its own name', () => {
  const book = ledger(math);
  const a = book.row(key(VALUE));
  const b = book.row(key({ ...VALUE, code: '3', measure: 'fills' }));
  book.read(a, 12, DEEP.cells);
  book.read(b, 12, DEEP.cells);
  const one = panels(num, a, b, VALUE);
  expect(one).toHaveLength(1);
  expect(one[0].rule.recurrence).toBe('a(n) = 11 a(n-1) - 24 a(n-2)');
  const two = panels(num, a, b, { ...VALUE, mix: 1, op: 'add' });
  expect(two[1].row.title).toBe('add of carpet 7 surface and tree 3 fills');
  expect(two[1].row.terms[0]).toBe(String(BigInt(a.terms[0]) + BigInt(b.terms[0])));
  expect(two[1].mate).toBe(b);
  const alone = panels(num, a, b, { ...VALUE, mix: 1, op: 'shift', arg: 2 });
  expect([alone[1].row.title, alone[1].row.terms[0], alone[1].mate]).toEqual(['shift of carpet 7 surface, argument 2', a.terms[2], null]);
});
