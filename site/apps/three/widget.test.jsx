import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { Frame } from '../../lib/frame.jsx';
import { defaults } from '../../lib/knobs.js';
import { Knobs } from '../../lib/knobs.jsx';
import { SPEC } from './scene.js';
import { Widget, hands } from './widget.jsx';

const value = { ...defaults(SPEC), seed: 1, base: 2, code: '23' };

const page = (unit, three) => renderToStaticMarkup(<Frame label="Three"><Widget value={value} onChange={() => {}} unit={unit} three={three} /></Frame>);

const knobs = (patch) => renderToStaticMarkup(<Knobs spec={SPEC} value={{ ...value, ...patch }} units={{ math }} onChange={() => {}} />);

const folds = (html) => [...html.matchAll(/<summary><h3>([^<]*)<\/h3>/g)].map((found) => found[1]);

test('the stage mounts once the unit and three are both in hand', () => {
  expect([page(undefined).includes('<canvas'), page(math).includes('<canvas'), page(math, {}).includes('<canvas')]).toEqual([false, false, true]);
});

test("the level runs to the demo's reach of each number, and a source shows only its own groups", () => {
  const max = (html) => /<span>Level<\/span><span class="num">\d+<\/span><input type="range" min="1" max="(\d)"/.exec(html)?.[1];
  expect([2, 3, 4, 5].map((number) => max(knobs({ number })))).toEqual(['7', '4', '3', '3']);
  expect(folds(knobs({}))).toEqual(['Solid', 'Crop', 'Circle', 'Look']);
  expect(folds(knobs({ source: 'field' }))).toEqual(['Solid', 'Crop', 'Field', 'Planes', 'Colour', 'Look']);
});

test('Space is the spin: the scene toggles the spin knob and plays while it is on', () => {
  const asked = [];
  const on = hands(() => ({ ...value, spin: 1 }), (patch) => asked.push(patch), math);
  const off = hands(() => ({ ...value, spin: 0 }), (patch) => asked.push(patch), math);
  on.toggle();
  off.toggle();
  expect([asked, on.playing(), off.playing()]).toEqual([[{ spin: 0 }, { spin: 1 }], true, false]);
});
