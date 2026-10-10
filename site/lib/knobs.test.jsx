import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { Btn, Export, Knob, Knobs, Toggle } from './knobs.jsx';

const SPEC = [
  { key: 'max', label: 'Jump speed', kind: 'slider', def: 14, min: 4, max: 30, step: 1, group: 'Jump' },
  { key: 'stars', label: 'Density', kind: 'slider', def: 1, min: 0.25, max: 3, step: 0.25, group: 'Sky' },
  { key: 'hyper', kind: 'segment', def: 'mix', options: ['no', 'yes', 'mix'], group: 'Saver' },
  { key: 'idle', label: 'Cruise', kind: 'slider', def: 0.015, min: 0, max: 0.1, step: 0.005, group: 'Jump' },
];

const LIVE = [
  { key: 'dim', label: 'Dim', kind: 'segment', def: 2, options: [2, 3], group: 'Design' },
  { key: 'level', label: 'Level', kind: 'slider', def: 3, min: 1, max: (value) => (value.dim === 3 ? 3 : 5), step: 1, group: 'Design' },
  { key: 'kind', label: 'Kind', kind: 'segment', def: 'moire', options: ['moire', 'star'], group: 'Stack' },
  { key: 'combine', label: 'Combine', kind: 'pick', def: 'sum', options: (value) => (value.kind === 'moire' ? [['sum', 'Sum'], ['hive', 'Hive']] : [['sum', 'Sum']]), group: 'Stack' },
  { key: 'rule', label: 'Rule', kind: 'slider', def: 110, min: 0, max: 255, step: 1, group: 'Rule', when: (value) => value.kind === 'star' },
  { key: 'text', label: 'Text', kind: 'text', def: '', wide: true, group: 'Text' },
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

test('a slider or number row with zero reads that word at 0 in its label alone, the unit dropped and the input kept at 0', () => {
  const spec = [
    { key: 'cell', label: 'Cell', kind: 'slider', def: 0, min: 0, max: 32, step: 1, unit: 'px', zero: 'Fit' },
    { key: 'ship', label: 'Ship', kind: 'number', def: 0, min: 0, max: 255, step: 1, zero: 'Random' },
  ];
  const read = (value) => [...renderToStaticMarkup(<Knobs spec={spec} value={value} onChange={() => {}} />).replaceAll('<!-- -->', '').matchAll(/<span class="num">([^<]+)<\/span>|value="([^"]*)"/g)].map((found) => found[1] ?? found[2]);
  expect([read({ cell: 0, ship: 0 }), read({ cell: 8, ship: 3 })]).toEqual([['Fit', '0', 'Random', '0'], ['8 px', '8', '3']]);
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

test('Knobs hides a row whose when is false and skips the group it empties', () => {
  const groups = (value) => [...renderToStaticMarkup(<Knobs spec={LIVE} value={value} onChange={() => {}} />).matchAll(/<h3>([^<]+)<\/h3>/g)].map((found) => found[1]);
  expect([groups({ kind: 'moire' }), groups({ kind: 'star' })]).toEqual([['Design', 'Stack', 'Text'], ['Design', 'Stack', 'Rule', 'Text']]);
});

test('a slider whose max follows the value shows that max, a pick whose options follow it lists those, and a text row reads wide', () => {
  const html = (value) => renderToStaticMarkup(<Knobs spec={LIVE} value={value} onChange={() => {}} />);
  const max = (text) => /type="range" min="1" max="(\d)"/.exec(text)[1];
  expect([max(html({ dim: 2 })), max(html({ dim: 3 }))]).toEqual(['5', '3']);
  const options = (text) => [...text.matchAll(/<option value="([^"]*)"/g)].map((found) => found[1]);
  expect([options(html({ kind: 'moire' })), options(html({ kind: 'star' }))]).toEqual([['sum', 'hive'], ['sum']]);
  expect(html({})).toContain('<input type="text" class="wide"');
});

test('a Knob writes its value tidied against the whole value, so a level past the cap of the dim lands on the cap', () => {
  const wrote = [];
  const level = LIVE[1];
  Knob({ row: level, value: 2, values: { dim: 3, level: 2 }, onChange: (patch) => wrote.push(patch) }).props.onChange(5);
  Knob({ row: level, value: 2, onChange: (patch) => wrote.push(patch) }).props.onChange(5);
  expect(wrote).toEqual([{ level: 3 }, { level: 5 }]);
});
