import { expect, test } from 'bun:test';
import * as num from '@mrlyprod/mrly/num';
import { REACH, ZEROS, fold, reading, study } from './engine.js';
import { rows, scrub } from './widget.jsx';

test('the readouts are the view shown: the head, its zeros, zeta at it and the first zeros marked passed, or the stairs at x', () => {
  const said = { t: 30, walk: reading(num, study(num), 30), stairs: fold(num, 100).facts(10) };
  const names = (view) => rows(view, said).map(([name]) => name);
  expect(names('walk').slice(0, 8)).toEqual(['t', 'zeros below t', 'next zero', 'Z(t)', 're', 'im', 'theta', 'zero 1']);
  expect(names('stairs')).toEqual(['x', 'zeros', 'psi(x)', 'formula', 'gap']);
  expect(Object.fromEntries(rows('walk', said))).toMatchObject({ t: '30.00', 'zeros below t': '3', 'next zero': '30.424876', 'zero 1': '14.134725 passed', 'zero 4': '30.424876' });
  expect(Object.fromEntries(rows('stairs', said))).toMatchObject({ x: '100', zeros: '10' });
  expect(rows('walk', null)).toEqual([]);
});

test('a drag across the walk scrubs t by half the reach a width, and across the stairs the zeros by all of them', () => {
  const grab = { w: 400, t: 30, zeros: 10 };
  expect(scrub({ ...grab, view: 'walk' }, 200)).toEqual({ t: 30 + REACH / 4 });
  expect(scrub({ ...grab, view: 'walk' }, -40)).toEqual({ t: 30 - REACH / 20 });
  expect(scrub({ ...grab, view: 'stairs' }, 100)).toEqual({ zeros: 10 + ZEROS / 4 });
});
