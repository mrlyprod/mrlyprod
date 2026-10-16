import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { Frame } from '../../lib/frame.jsx';
import { plan } from './scene.js';
import { Widget, filters, rows } from './widget.jsx';

const value = { seed: 1, dim: 2, base: 2, code: '', level: 3, tint: 'degree', spin: 0, show: 'classes' };

test('the widget mounts its stage once the unit is in hand, and a cube only once three is too', () => {
  const page = (unit, three, patch = {}) => renderToStaticMarkup(<Frame label="Designs"><Widget value={{ ...value, ...patch }} onChange={() => {}} unit={unit} three={three} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
  expect(page(math, undefined, { dim: 3 })).not.toContain('<canvas');
  expect(page(math, {}, { dim: 3 })).toContain('<canvas');
});

test('the gallery shows the classes unless the url asks for all or the named, and only the named where the space is too big to list', () => {
  expect([filters(math, 2, 2, undefined).shown, filters(math, 3, 2, 'all').shown, filters(math, 2, 2, 'junk').shown, filters(math, 3, 3, 'classes').shown]).toEqual(['classes', 'all', 'classes', 'named']);
});

test('the readouts name the design, its class, orbit, form and fill out of the grid, and the distinct counts by dimension', () => {
  const read = (patch) => Object.fromEntries(rows(plan(math, { ...value, ...patch }).info));
  expect(read({ code: '7' })).toMatchObject({ Design: 'carpet 7', Orbit: '4', Degree: '2', 'Normal form': '1+xy', Filled: '512 of 729', Dimension: (Math.log(8) / Math.log(3)).toFixed(4), 'Distinct, dim 1 to 4': '3, 6, 22, 402' });
  expect([read({ code: '7' }).Class, read({ code: '11' }).Class, read({ base: 3, code: '495' }).Degree]).toEqual([undefined, '7', undefined]);
});
