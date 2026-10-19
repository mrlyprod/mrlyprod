import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import * as num from '@mrlyprod/mrly/num';
import { Frame } from '../../lib/app/frame.jsx';
import { defaults } from '../../lib/app/spec.js';
import { Accordion } from '../../lib/app/controls.jsx';
import { Knobs, useAccordion } from '../../lib/app/knobs.jsx';
import { PRESETS, dial, preset, study } from './engine.js';
import { SPEC } from './scene.js';
import { Chips, Pane, Widget, deeper, degrees, next, rows } from './widget.jsx';

const value = { ...defaults(SPEC), seed: 1 };

const flat = (html) => html.replaceAll('<!-- -->', '');

function Shared({ children }) {
  return <Accordion.Provider value={useAccordion('', SPEC)}>{children}</Accordion.Provider>;
}

const order = (html) => [...html.matchAll(/<(?:section|details)([^>]*)>(?:<summary>)?(?:<h3>([^<]*)<\/h3>)?/g)].filter(([, attrs]) => attrs.includes('order:')).map(([, attrs, name]) => [Number(attrs.match(/order:(\d+)/)[1]), name ?? '']).sort(([a], [b]) => a - b).map(([, name]) => name);

test('the stage is named for the picture it draws', () => {
  const page = (units) => renderToStaticMarkup(<Frame label="Radix"><Widget value={value} onChange={() => {}} units={units} /></Frame>);
  expect(page({ math, num })).toContain('<canvas role="img" aria-label="The level 5 words of the radix design');
});

test('a chip a class, the held ones pressed and labelled by their own digit, the last held one locked', () => {
  const html = flat(renderToStaticMarkup(<Chips dialed={dial(num, math, value)} onToggle={() => {}} />));
  expect(html.match(/<button /g)).toHaveLength(9);
  expect(html.match(/aria-pressed="true"/g)).toHaveLength(4);
  expect(html).toContain('title="class 4: digit 2 held"');
  const lone = flat(renderToStaticMarkup(<Chips dialed={dial(num, math, { ...value, d: '1:0', t: '0' })} onToggle={() => {}} />));
  expect(lone.match(/disabled=""/g)).toHaveLength(1);
});

const pane = (over) => {
  const one = { ...value, ...over };
  const moves = { flat: () => {}, every: () => {} };
  return flat(renderToStaticMarkup(<Shared><Pane value={one} units={{ math, num }} dialed={dial(num, math, one)} facts={study(num, math, one).facts(one.level)} name="" label="Custom" onChange={() => {}} change={() => {}} moves={moves} /><Knobs spec={SPEC} value={one} onChange={() => {}} /></Shared>));
};

test('the pane reads the design, the twists and the base, the box only on base 3 of the square lattice, then the look, with a twist pick a held digit', () => {
  const koch = pane();
  expect([order(koch), order(pane(preset(num, math, 'carpet')))]).toEqual([['Design', 'Twists', 'Base', 'Look'], ['Design', 'Twists', 'Base', 'Box', 'Look']]);
  expect(koch.match(/<span>Twist on [^<]+<\/span>/g)).toEqual(['<span>Twist on 0</span>', '<span>Twist on 1</span>', '<span>Twist on 2 + w</span>', '<span>Twist on 2</span>']);
  expect(koch).toContain('<span>Level 1 to 8</span>');
});

test('the readouts go to the status: the fill, the distinct count, the glue, and an error row only on an error', () => {
  const facts = study(num, math, value).facts(5);
  expect(rows(facts).slice(0, 4)).toEqual([['level', '5 of 8'], ['fill', 1024], ['distinct', 1024], ['glue', 'none, one point a word']]);
  expect(rows(facts).find(([name]) => name === 'dimension')[1]).toBe('1.261860');
  expect(rows({ ...facts, error: 'boom' }).at(-1)).toEqual(['error', 'boom']);
  expect(rows(null)).toEqual([]);
});

test('the arrows step round the presets, a custom design stepping to an end', () => {
  expect([next('koch', 1), next('koch', -1), next('', 1), next('', -1)]).toEqual([PRESETS[1][0], PRESETS.at(-1)[0], PRESETS[0][0], PRESETS.at(-1)[0]]);
});

test('a tap goes one level deeper and wraps to the first at the cap', () => {
  expect([deeper({ ...value, level: 5 }), deeper({ ...value, level: 8 })]).toEqual([6, 1]);
});

test('a twist reads as its turn in degrees', () => {
  expect([degrees(0, 6), degrees(1, 6), degrees(3, 4)]).toEqual(['0°', '60°', '270°']);
});

test('the Cell knob reads Fit at 0 and its pixels above it', () => {
  const cell = (n) => flat(renderToStaticMarkup(<Knobs spec={SPEC} value={{ ...value, cell: n }} onChange={() => {}} />)).match(/<span>Cell<\/span><span class="num">([^<]+)<\/span>/)[1];
  expect([cell(0), cell(8)]).toEqual(['Fit', '8 px']);
});
