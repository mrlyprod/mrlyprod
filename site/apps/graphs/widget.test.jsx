import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { defaults } from '../../lib/knobs.js';
import { Frame } from '../../lib/frame.jsx';
import { Accordion, Knobs, useAccordion } from '../../lib/knobs.jsx';
import { SPEC } from './scene.js';
import { Pane, Widget, note, rows } from './widget.jsx';

const value = { ...defaults(SPEC), seed: 1 };

const page = (unit, three, patch = {}) => renderToStaticMarkup(<Frame label="Graphs"><Widget value={{ ...value, ...patch }} onChange={() => {}} units={unit && { math: unit }} three={three} /></Frame>);

const panel = (patch = {}, relax = null) => <Pane value={{ ...value, ...patch }} onChange={() => {}} math={math} relax={relax} onRelax={() => {}} onReset={() => {}} onPose={() => {}} />;

const pane = (patch, relax) => renderToStaticMarkup(panel(patch, relax));

function Shared({ children }) {
  return <Accordion.Provider value={useAccordion('', SPEC)}>{children}</Accordion.Provider>;
}

const order = (html) => [...html.matchAll(/<(?:section|details)([^>]*)>(?:<summary>)?(?:<h3>([^<]*)<\/h3>)?/g)].filter(([, attrs]) => attrs.includes('order:')).map(([, attrs, name]) => [Number(attrs.match(/order:(\d+)/)[1]), name ?? '']).sort(([a], [b]) => a - b).map(([, name]) => name);

const placed = (patch) => order(renderToStaticMarkup(<Shared>{panel(patch)}<Knobs spec={SPEC} value={{ ...value, ...patch }} onChange={() => {}} /></Shared>));

test('the widget mounts its stage once the unit is in hand, and a cube only once three is too', () => {
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
  expect(page(math, undefined, { view: 'cube' })).not.toContain('<canvas');
  expect(page(math, {}, { view: 'cube' })).toContain('<canvas');
});

test('the pane holds the design picker on the view\'s own code, Relax or Pause beside Reset, and the three poses', () => {
  const html = pane();
  expect(html.startsWith('<details class="group"><summary><h3>Design</h3><span class="live">495</span>')).toBe(true);
  expect(html).toContain('value="495"');
  expect(pane({ view: 'hex' })).toContain('value="23"');
  expect(pane({ layout: 'force' }, { on: false })).toContain('>Relax</button><button type="button">Reset</button>');
  expect(pane({ layout: 'force' }, { on: true })).toContain('>Pause</button>');
  expect(['Corner', 'Edge', 'Face'].every((word) => pane({ view: 'cube' }).includes(word))).toBe(true);
});

test('the pane reads the view rows, the design, then the layout with relax and reset on a force layout, the camera with its poses on a cube, and the look', () => {
  expect([placed({}), placed({ layout: 'force', view: 'cube' })]).toEqual([['', 'Design', 'Layout', 'Look'], ['', 'Design', 'Layout', 'Relax', 'Camera', 'Pose', 'Look']]);
});

test('the status names the design and the network, its counts to the demo\'s places, and the relax readouts on a force layout', () => {
  const facts = { name: 'carpet', code: '495', space: 'flat', graph: 'core', level: 2, layout: 'lattice', nodes: 64, branches: 88, tips: 0, junctions: 44, pieces: 1, length: 88, box: 1.5700439, euler: null };
  expect(rows(facts, null)).toEqual([['Design', 'carpet 495'], ['Network', 'core graph, level 2, lattice'], ['Nodes', 64], ['Branches', 88], ['Tips', 0], ['Junctions', 44], ['Pieces', 1], ['Length', '88.00'], ['Box dimension', '1.570'], ['Euler', 'none']]);
  expect(rows(facts, { ticks: 0, energy: 0, moved: 0, heat: 2.7 }).slice(10)).toEqual([['Ticks', 0], ['Heat', '2.70e+0']]);
  expect(rows(facts, { ticks: 40, energy: 0.5, moved: 0.01, heat: 0.2 }).slice(10).map(([name]) => name)).toEqual(['Ticks', 'Energy', 'Moved', 'Heat']);
  expect([note({ ...facts, name: '', space: 'cube' }, 'iso'), note({ ...facts, space: 'cube' })]).toEqual(['core graph, level 2, lattice, isometric', 'core graph, level 2, lattice, perspective']);
});
