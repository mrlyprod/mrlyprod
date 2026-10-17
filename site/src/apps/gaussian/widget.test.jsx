import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as num from '@mrlyprod/mrly/num';
import { Frame } from '../../lib/app/frame.jsx';
import { defaults } from '../../lib/app/spec.js';
import { Knobs } from '../../lib/app/knobs.jsx';
import { rng } from '../../lib/rand.js';
import { hit, shuffle, study } from './engine.js';
import { SPEC } from './scene.js';
import { Widget, rerolled, rows } from './widget.jsx';

const value = { ...defaults(SPEC), seed: 1, limit: 12 };

const labels = (list) => list.map(([label]) => label);

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Gaussian"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(num)).toContain('<canvas');
});

test('the page opens on the gaussian window of reach 40 coloured by fate, every window knob in the first group', () => {
  const html = renderToStaticMarkup(<Knobs spec={SPEC} value={defaults(SPEC)} onChange={() => {}} />);
  expect([defaults(SPEC).ring, defaults(SPEC).limit, defaults(SPEC).look, defaults(SPEC).units]).toEqual(['gaussian', 40, 'fate', 1]);
  expect(SPEC.filter((row) => row.group === 'Window').map((row) => row.key)).toEqual(['ring', 'limit', 'look', 'tint', 'faint']);
  for (const label of ['Ring', 'Reach', 'Colour', 'Composites', 'Show its units']) expect(html).toContain(label);
  expect(html).not.toContain('>Tint<');
});

test('the Cell knob reads Fit at 0, not 0 px, and its pixels once pinned', () => {
  const read = (cell) => renderToStaticMarkup(<Knobs spec={SPEC} value={{ ...value, cell }} onChange={() => {}} />).replaceAll('<!-- -->', '').match(/<span>Cell<\/span><span class="num">([^<]+)<\/span>/)[1];
  expect([read(0), read(8)]).toEqual(['Fit', '8 px']);
});

test('a reroll writes a shuffled ring, reach and colour from the new seed and lets the point go', () => {
  expect(rerolled(7)).toEqual({ ...shuffle(rng(7)), click: '' });
});

test('the readouts lead with the clicked point, then the window, a missed tap first, and the colour key only under fate', () => {
  const plan = study(num, 'gaussian', 12);
  const window = rows(plan.facts, null, false, 'fate');
  expect(labels(window)).toEqual(['window', 'primes', 'density', 'split', 'inert', 'ramified', 'points', 'symmetry', 'busiest norm', 'key']);
  expect(window[0][1]).toBe('gaussian, reach 12, norms to 288, 4 units');
  expect(window[2][1]).toMatch(/^\d+\.\d\d%$/);
  const split = rows(plan.facts, hit(num, plan, 2, 3), true, 'norm');
  expect(labels(split).slice(0, 9)).toEqual(['tap', 'point', 'verdict', 'norm', 'class', 'factors', 'conjugate', 'associates', 'points of norm 13']);
  expect(split.slice(1, 3)).toEqual([['point', '2 + 3i at 2, 3'], ['verdict', 'prime: 13 splits as (2 + 3i)(2 - 3i)']]);
  expect(labels(split)).not.toContain('key');
  expect(labels(rows(plan.facts, hit(num, plan, 0, 0), false, 'plain')).slice(0, 3)).toEqual(['point', 'verdict', 'window']);
  expect(rows(undefined, null, false, 'fate')).toEqual([]);
});
