import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { Frame } from '../../lib/frame.jsx';
import { defaults } from '../../lib/knobs.js';
import { Accordion, Knobs, useAccordion } from '../../lib/knobs.jsx';
import { studio } from './engine.js';
import { SPEC } from './scene.js';
import { Pane, Presets, Widget, label, rows } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const VALUE = { ...defaults(SPEC), seed: 1, base: 2, code: '23' };

function Shared({ children }) {
  return <Accordion.Provider value={useAccordion('', SPEC)}>{children}</Accordion.Provider>;
}

const order = (html) => [...html.matchAll(/<(?:section|details)([^>]*)>(?:<summary>)?(?:<h3>([^<]*)<\/h3>)?/g)].filter(([, attrs]) => attrs.includes('order:')).map(([, attrs, name]) => [Number(attrs.match(/order:(\d+)/)[1]), name ?? '']).sort(([a], [b]) => a - b).map(([, name]) => name);

test('the widget mounts its picture, labelled for a reader, only once the unit is in hand', () => {
  const page = (units) => renderToStaticMarkup(<Frame label="Six"><Widget value={VALUE} onChange={() => {}} units={units} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page({ math })).toContain('<canvas role="img" aria-label="a cube design repeated into a sheet"');
  expect(label({ name: 'carpet', title: 'bang dim 3, code 23' })).toBe('carpet repeated into a sheet');
});

test('the readouts are the demo stats of the sheet', () => {
  const facts = studio(math)(VALUE).facts;
  expect(rows(facts).map(([name]) => name)).toEqual(['design', 'side', 'level', 'copies', 'filled', 'exposed', 'one copy exposed', 'tile', 'sheet', 'triangles', 'empty', 'pieces', 'holes', 'corners', 'edges', 'euler']);
  expect(rows(facts).slice(0, 7)).toEqual([['design', 'carpet 23'], ['side', '9'], ['level', '2 of 2'], ['copies', '25'], ['filled', '7650'], ['exposed', '3042'], ['one copy exposed', '162']]);
  expect(rows(null)).toEqual([]);
});

test('the presets are the demo sheets as buttons, the one in use lit, and a tap writes a rectangle', () => {
  const html = renderToStaticMarkup(<Presets value={VALUE} onChange={() => {}} />);
  expect(html.match(/<button /g)).toHaveLength(2);
  expect(html).toContain('class="on">tile(5,5)<');
  expect(renderToStaticMarkup(<Presets value={{ ...VALUE, sheet: 'disc' }} onChange={() => {}} />)).not.toContain('class="on"');
  const wrote = [];
  Presets({ value: VALUE, onChange: (patch) => wrote.push(patch) }).props.children[1].props.onClick();
  expect(wrote).toEqual([{ sheet: 'rect', x: 3, y: 9 }]);
});

test('the pane reads the view, the design, the sheet and its presets, then the tile and the look', () => {
  const html = renderToStaticMarkup(<Shared><Pane value={VALUE} onChange={() => {}} /><Knobs spec={SPEC} value={VALUE} onChange={() => {}} /></Shared>);
  expect([order(html), html.includes('loading')]).toEqual([['', 'Design', 'Sheet', 'Presets', 'Tile', 'Look'], true]);
});
