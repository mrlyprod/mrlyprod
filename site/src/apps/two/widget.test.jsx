import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { Frame } from '../../lib/app/frame.jsx';
import { defaults, describe, tidy } from '../../lib/app/spec.js';
import { SPEC } from './scene.js';
import { Folds } from '../../lib/two/parts.jsx';
import { Widget, panes, rows } from './widget.jsx';

const value = { ...defaults(SPEC), seed: 1, base: 3, code: '495' };

const shown = (over) => ({ ...value, ...over, ...tidy(SPEC, { ...value, ...over }, { math }) });

const pane = (over) => renderToStaticMarkup(<Folds groups={panes(shown(over), () => {}, math)} />);

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Two"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
});

test('the widget folds hold the design with its side and level, then the sheet with its presets, and the look is a spec group', () => {
  const html = pane({});
  const order = ['<h3>Design</h3>', 'aria-label="Base"', '>Side<', '>Level<', '<h3>Sheet</h3>', '>Across<', '>5 by 5</button>', '>3 by 9</button>'].map((one) => html.indexOf(one));
  expect(order.every((at) => at >= 0)).toBe(true);
  expect(order).toEqual([...order].sort((a, b) => a - b));
  expect(html).toContain('class="on"');
  expect(describe(SPEC, value).map((group) => group.name)).toEqual(['Look']);
});

test('the level slider runs to the cap of the sheet and shows the level the sheet holds', () => {
  const slider = (over) => /<span>Level<\/span><span class="num">(\d+)<\/span><input type="range" min="1" max="(\d+)"/.exec(pane(over)).slice(1).map(Number);
  expect(slider({ level: 5, x: 1, y: 1 })).toEqual([5, 5]);
  expect(slider({ level: 5, x: 12, y: 12 })).toEqual([3, 3]);
});

test('the readouts lead with the design, the copies, what the sheet exposes and what one copy does', () => {
  const facts = { code: '495', name: 'carpet', copies: 25, exposed: 1280, one: 80, fills: 1600, voids: 425, side: 9, cols: 45, rows: 45, level: 2, corners: 1, edges: 2, euler: 3, dimension: 1.89 };
  const read = rows(facts);
  expect(read.slice(0, 4)).toEqual([['design', 'carpet 495'], ['copies', 25], ['exposed', 1280], ['one copy exposed', 80]]);
  expect(Object.fromEntries(read).dimension).toBe('1.8900');
});

test('a blank or junk code labels the design fold with the code the seed settles on', () => {
  for (const code of ['', 'abc', '-3', '1.5']) expect(pane({ code })).toContain('<h3>Design</h3>');
  expect(panes(shown({ code: '1.5' }), () => {}, math)[0].live).toBe('1 · side 3 · level 2');
});
