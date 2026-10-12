import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { all, find, mount, text } from './dom.js';
import { Accordion, Btn, Export, Fold, Icon, Knob, Knobs, Menu, Pick, Slider, Toggle, gist, useAccordion } from './knobs.jsx';

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

test('a slider input fires onChange on input, before any change event', () => {
  const wrote = [];
  const input = Knob({ row: SPEC[0], value: 14, onChange: (patch) => wrote.push(patch) }).props;
  expect(input.commit).toBeUndefined();
  Slider(input).props.children[2].props.onChange({ target: { value: '20' } });
  expect(wrote).toEqual([{ max: 20 }]);
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

test('a Btn or a fold header a pointer clicks lets go of focus before its action, one a key presses keeps it', () => {
  const seen = [];
  const click = Btn({ onClick: () => seen.push('act'), children: 'Reroll' }).props.onClick;
  const head = Fold({ name: 'Jump', open: false }).props.children[0].props.onClick;
  const at = (fn, detail) => fn({ detail, currentTarget: { blur: () => seen.push(`blur ${detail}`) } });
  at(click, 1);
  at(click, 0);
  at(head, 1);
  at(head, 0);
  expect(seen).toEqual(['blur 1', 'act', 'act', 'blur 1']);
});

test('Knobs folds each group: the first open, the rest shut, every summary naming the group and its live values', () => {
  const html = renderToStaticMarkup(<Knobs spec={SPEC} value={{ max: 20, stars: 1, hyper: 'mix', idle: 0 }} onChange={() => {}} />);
  const folds = [...html.matchAll(/<details class="group"( open="")?><summary><h3>([^<]+)<\/h3><span class="live">([^<]*)<\/span><svg/g)].map((found) => [found[2], found[3], Boolean(found[1])]);
  expect(folds).toEqual([['Jump', '20 · 0', true], ['Sky', '1', false], ['Saver', 'mix', false]]);
});

test('a fold summary ends on the expand_more chevron, after its live values', () => {
  const fold = renderToStaticMarkup(<Fold name="Jump" open={false} live="20">x</Fold>);
  expect(fold).toContain(`<span class="live">20</span>${renderToStaticMarkup(<Icon name="expand_more" />)}</summary>`);
});

test('a pick draws the expand_more chevron in a span after its select', () => {
  const pick = renderToStaticMarkup(<Pick label="Shape" value="a" options={['a', 'b']} onChange={() => {}} />);
  expect(pick).toContain(`</select><span class="drop">${renderToStaticMarkup(<Icon name="expand_more" />)}</span></label>`);
});

test('Knobs opens the groups its app kept in sessionStorage in place of the first', () => {
  globalThis.sessionStorage = { getItem: (key) => (key === 'knobs:sky' ? '["Sky"]' : null), setItem: () => {} };
  const open = (id) => [...renderToStaticMarkup(<Knobs spec={SPEC} value={{}} onChange={() => {}} id={id} />).matchAll(/<details class="group" open=""><summary><h3>([^<]+)<\/h3>/g)].map((found) => found[1]);
  expect([open('sky'), open('sea')]).toEqual([['Sky'], ['Jump']]);
  delete globalThis.sessionStorage;
});

test('a gist reads a zero word, a unit, an option label, an on toggle and a text, and skips an off toggle and an empty text', () => {
  const rows = [
    { key: 'cell', label: 'Cell', kind: 'slider', def: 0, min: 0, max: 32, unit: 'px', zero: 'Fit' },
    { key: 'every', label: 'Every', kind: 'slider', def: 6, min: 3, max: 30, unit: 's' },
    { key: 'shape', label: 'Shape', kind: 'pick', def: 'square', options: [['square', 'Square'], ['hex', 'Hex']] },
    { key: 'shimmer', label: 'Shimmer', kind: 'toggle', def: 1 },
    { key: 'outline', label: 'Outline', kind: 'toggle', def: 0 },
    { key: 'text', label: 'Text', kind: 'text', def: '' },
  ];
  expect([gist(rows, { cell: 0, every: 6, shape: 'hex', shimmer: 1, outline: 0, text: '' }), gist(rows, { cell: 8, every: 12, shape: 'square', shimmer: 0, outline: 1, text: 'hi' })]).toEqual(['Fit · 6 s · Hex · Shimmer', '8 px · 12 s · Square · Outline · hi']);
});

test('Export is a dialog: PNG and WebP, the sizes 1x, 2x and 4x only for a scene that can fix its frame, a Save button, then the rows of more in order', () => {
  const html = (fix) => renderToStaticMarkup(<Export canvas={null} name="x" fix={fix} more={[['Video 9:16', () => {}], ['Video 16:9', () => {}]]} />);
  const radios = (text) => [...text.matchAll(/type="radio" name="[^"]*"[^>]*? value="([^"]+)"/g)].map((found) => found[1]);
  expect(html(() => {})).toContain('<dialog class="modal export" aria-label="Export">');
  expect([radios(html(() => {})), radios(html())]).toEqual([['png', 'webp', '1', '2', '4'], ['png', 'webp']]);
  expect([...html().matchAll(/<button type="button"( class="primary")?>([^<]+)<\/button>/g)].map((found) => found[2])).toEqual(['Save PNG', 'Video 9:16', 'Video 16:9']);
});

test('a Menu is a button and a popover listing its rows in order, a row with an icon drawing it before its text', () => {
  const html = renderToStaticMarkup(<Menu className="icon more" aria-label="More" rows={[['Copy link', () => {}, 'link'], ['Zen', () => {}]]}>x</Menu>);
  expect(html).toMatch(/<button type="button" class="icon more" popoverTarget="(menu[^"]+)"[^>]*aria-label="More">x<\/button><div id="\1" popover="auto" class="pop"/);
  expect([...html.matchAll(/<button type="button">(<svg[^]*?<\/svg>)?([^<]+)<\/button>/g)].map((found) => [found[2], Boolean(found[1])])).toEqual([['Copy link', true], ['Zen', false]]);
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

test('a slider row with show reads its number through it, in the knob and the gist, and a show that answers no text leaves the number', () => {
  const rows = [
    { key: 't', label: 'T', kind: 'slider', def: 30, min: 0, max: 99, step: 0.01, show: (v) => v.toFixed(2) },
    { key: 'density', label: 'Density', kind: 'slider', def: 0.3, min: 0, max: 1, step: 0.01, show: (value) => value.from === 'noise' },
  ];
  const nums = [...renderToStaticMarkup(<Knobs spec={rows} value={{ t: 30, density: 0.3 }} onChange={() => {}} />).matchAll(/<span class="num">([^<]+)<\/span>/g)].map((found) => found[1]);
  expect([nums, gist(rows, { t: 30, density: 0.3 })]).toEqual([['30.00', '0.3'], '30.00 · 0.3']);
});

test('a row label and unit may be functions of the value and the units, read by the knob, a toggle gist and a slider gist', () => {
  const rows = [
    { key: 'level', label: (value, units) => `Level 1 to ${units.top}`, kind: 'slider', def: 2, min: 1, max: 8, step: 1, unit: (value, units) => `of ${units.top}` },
    { key: 'seams', label: (value) => (value.level > 1 ? 'Seams' : 'Seam'), kind: 'toggle', def: 1 },
  ];
  const html = renderToStaticMarkup(<Knobs spec={rows} value={{ level: 2, seams: 1 }} units={{ top: 4 }} onChange={() => {}} />).replaceAll('<!-- -->', '');
  expect([[...html.matchAll(/<span>([^<]+)<\/span>|<span class="num">([^<]+)<\/span>/g)].map((found) => found[1] ?? found[2]), gist(rows, { level: 2, seams: 1 }, { top: 4 })]).toEqual([['Level 1 to 4', '2 of 4', 'Seams'], '2 of 4 · Seams']);
});

test('the first fold of the pane opens: the view group, else a widget fold not placed after a group, else the first spec group; a kept set wins', async () => {
  const VIEW = [...SPEC, { key: 'view', kind: 'segment', def: 'a', options: ['a', 'b'], group: 'Saver' }];
  const Pane = ({ spec, widget }) => {
    const folds = useAccordion('sky', spec);
    return (
      <Accordion.Provider value={folds}>
        {widget}
        <Knobs spec={spec} value={{}} onChange={() => {}} />
      </Accordion.Provider>
    );
  };
  const open = async (kept, spec, widget) => {
    globalThis.sessionStorage = { getItem: () => kept, setItem: () => {} };
    try {
      return await mount(async (draw, host) => {
        await draw(<Pane spec={spec} widget={widget} />);
        return all(host, (at) => at.tagName === 'DETAILS' && 'open' in at.attributes).map((at) => text(find(at, (one) => one.tagName === 'H3')));
      });
    } finally {
      delete globalThis.sessionStorage;
    }
  };
  const source = <Fold name="Source">x</Fold>;
  expect([await open(null, SPEC, source), await open(null, SPEC, null), await open('["Sky"]', SPEC, source), await open(null, VIEW, source), await open(null, SPEC, <Fold name="Source" after="Sky">x</Fold>)]).toEqual([['Source'], ['Jump'], ['Sky'], ['Saver'], ['Jump']]);
});
