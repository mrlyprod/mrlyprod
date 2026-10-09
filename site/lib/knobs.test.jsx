import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { Btn, Export, Knobs, Toggle } from './knobs.jsx';

const SPEC = [
  { key: 'max', label: 'Jump speed', kind: 'slider', def: 14, min: 4, max: 30, step: 1, group: 'Jump' },
  { key: 'stars', label: 'Density', kind: 'slider', def: 1, min: 0.25, max: 3, step: 0.25, group: 'Sky' },
  { key: 'hyper', kind: 'segment', def: 'mix', options: ['no', 'yes', 'mix'], group: 'Saver' },
  { key: 'idle', label: 'Cruise', kind: 'slider', def: 0.015, min: 0, max: 0.1, step: 0.005, group: 'Jump' },
];

test('Knobs draws the groups of a spec in first-seen order, each row under its group', () => {
  const html = renderToStaticMarkup(<Knobs spec={SPEC} value={{}} onChange={() => {}} />);
  const seen = [...html.matchAll(/<h3>([^<]+)<\/h3>|<span>(Jump speed|Cruise|Density|hyper)<\/span>/g)].map((found) => found[1] ?? found[2]);
  expect(seen).toEqual(['Jump', 'Jump speed', 'Cruise', 'Sky', 'Density', 'Saver', 'hyper']);
});

test('a Toggle writes 1 when switched on and 0 when switched off', () => {
  const wrote = [];
  const input = (value) => Toggle({ label: 'wrap', value, onChange: (v) => wrote.push(v) }).props.children[1];
  input(0).props.onChange({ target: { checked: true } });
  input(1).props.onChange({ target: { checked: false } });
  expect(wrote).toEqual([1, 0]);
});

test('a slider row with a unit shows the unit after its number, one without shows the number alone', () => {
  const spec = [
    { key: 'tick', label: 'Tick', kind: 'slider', def: 150, min: 50, max: 500, step: 25, unit: 'ms' },
    { key: 'idle', label: 'Cruise', kind: 'slider', def: 0.015, min: 0, max: 0.1, step: 0.005 },
  ];
  const html = renderToStaticMarkup(<Knobs spec={spec} value={{}} onChange={() => {}} />).replaceAll('<!-- -->', '');
  expect([...html.matchAll(/<span class="num">([^<]+)<\/span>/g)].map((found) => found[1])).toEqual(['150 ms', '0.015']);
});

test('a list keeps its last switch on: the one left is disabled, two on are both free', () => {
  const spec = [{ key: 'set', kind: 'list', def: 'lds', sep: '', options: [['l', 'Letters'], ['d', 'Digits'], ['s', 'Specials']] }];
  const switches = (set) => [...renderToStaticMarkup(<Knobs spec={spec} value={{ set }} onChange={() => {}} />).matchAll(/<input[^>]*>/g)].map(([tag]) => [/checked/.test(tag), /disabled/.test(tag)]);
  expect([switches('d'), switches('ld')]).toEqual([[[false, false], [true, true], [false, false]], [[true, false], [true, false], [false, false]]]);
});

test('a Btn a pointer clicks lets go of focus after its action, one a key presses keeps it', () => {
  const seen = [];
  const click = Btn({ onClick: () => seen.push('act'), children: 'Reroll' }).props.onClick;
  const at = (detail) => click({ detail, currentTarget: { blur: () => seen.push(`blur ${detail}`) } });
  at(1);
  at(0);
  expect(seen).toEqual(['act', 'blur 1', 'act']);
});

test('Export lists PNG and WebP, then the rows of more in order', () => {
  const html = renderToStaticMarkup(<Export canvas={null} name="x" more={[['Video 9:16', () => {}], ['Video 16:9', () => {}]]} />);
  expect([...html.matchAll(/<button type="button">([^<]+)<\/button>/g)].map((found) => found[1])).toEqual(['PNG', 'WebP', 'Video 9:16', 'Video 16:9']);
});
