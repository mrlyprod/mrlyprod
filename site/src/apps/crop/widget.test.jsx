import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { Frame } from '../../lib/app/frame.jsx';
import { defaults, tidy } from '../../lib/app/spec.js';
import { Knobs } from '../../lib/app/knobs.jsx';
import { Folds } from '../../lib/two/parts.jsx';
import { SPEC } from './scene.js';
import { Widget, panes, rows } from './widget.jsx';

const value = { ...defaults(SPEC), seed: 1, base: 2, code: '7' };

const shown = (over) => ({ ...value, ...over, ...tidy(SPEC, { ...value, ...over }, { math }) });

const pane = (over) => renderToStaticMarkup(<Folds groups={panes(shown(over), () => {}, math)} />);

const knobs = (over) => renderToStaticMarkup(<Knobs spec={SPEC} value={shown(over)} units={{ math }} onChange={() => {}} />);

const slider = (html, label) => new RegExp(`<span>${label}</span><span class="num">(\\d+)[^<]*</span><input type="range" min="1" max="(\\d+)"`).exec(html).slice(1).map(Number);

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Crop"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
});

test('the design fold holds the picker, the number and the level, and the spec groups follow: shape, look, then the circle count behind its toggle', () => {
  const design = pane({});
  const order = ['<h3>Design</h3>', 'aria-label="Base"', '>Number<', '>Level<'].map((one) => design.indexOf(one));
  expect(order.every((at) => at >= 0)).toBe(true);
  expect(order).toEqual([...order].sort((a, b) => a - b));
  const rest = knobs({ circle: 1 });
  const after = ['<h3>Shape</h3>', '>Radius<', 'aria-label="Mode"', '>Policy<', '>Exact edge<', '<h3>Look</h3>', '>Tint<', '<h3>Circle</h3>', '>Count<', 'aria-label="Centre"', '>Count level<', '>Radius r<'].map((one) => rest.indexOf(one));
  expect(after.every((at) => at >= 0)).toBe(true);
  expect(after).toEqual([...after].sort((a, b) => a - b));
  expect(rest).not.toContain('<h3>Design</h3>');
  expect(knobs({})).not.toContain('>Count level<');
});

test('the level, count and r sliders run to their caps and show what the picture holds', () => {
  const big = { number: 7, level: 5, count: 9, r: 700, circle: 1 };
  expect(slider(pane(big), 'Level')).toEqual([2, 2]);
  expect(slider(knobs(big), 'Count level')).toEqual([3, 3]);
  expect(slider(knobs(big), 'Radius r')).toEqual([342, 342]);
  expect(knobs(big)).toContain('<span class="num">342 /342</span>');
  expect(slider(pane({ level: 5 }), 'Level')).toEqual([5, 5]);
});

test('the readouts lead with the design and its census, and with the circle count while it is on', () => {
  const facts = { code: '7', name: 'carpet', in: 332, cut: 76, out: 104, before: 496, after: 400, side: 27, note: 'touching', level: 3, circle: null };
  expect(rows(facts).slice(0, 4)).toEqual([['design', 'carpet 7'], ['in', 332], ['cut', 76], ['out', 104]]);
  const read = rows({ ...facts, circle: { side: 729, r: 27, seen: 385, inside: 363, cut: 42, d: 1.8927892607, mass: 8 } });
  expect(read.slice(0, 5)).toEqual([['N(r)', 385], ['inside(r)', 363], ['C(r)', 42], ['count side', 729], ['design', 'carpet 7']]);
  expect(Object.fromEntries(read)).toMatchObject({ d: '1.8927893', 'fill of one tile': 8 });
});
