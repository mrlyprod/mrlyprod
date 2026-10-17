import { expect, test } from 'bun:test';
import { KINDS } from './chart.js';
import { pen, said } from '../../../test/pen.js';
import { KIND, chart, drawRibbon, fit, glyph, growth, legend, ribbon, shade, slotAt, tones, wrap } from './paint.js';

const ROWS = [
  { key: '2-2-7-fills-level', label: 'carpet 7', measure: 'fills', axis: 'level', cells: [1, 1, 1, 1, 0, 1, 1, 1, 1], terms: ['8', '64', '512', '4096', '32768'] },
  { key: '2-2-7-euler-level', label: 'carpet 7', measure: 'euler', axis: 'level', terms: ['0', '-8', '-72', '-680', '-4680'] },
];

const VIEW = { look: () => ({ paper: '#ffffff', accent: '#008cff' }), dpr: 1, still: false, t: 0, w: 800, h: 500 };

const BOX = { x: 40, y: 20, w: 600, h: 300 };

test('rows wear their own shades of the accent', () => {
  expect(shade('#008cff', '#ffffff', 0, 3)).toBe('rgb(0, 140, 255)');
  expect(shade('#008cff', '#ffffff', 2, 3)).toBe('rgb(158, 211, 255)');
  expect(shade('#008cff', '#ffffff', 0, 1)).toBe('rgb(0, 140, 255)');
});

test('every chart kind draws the rows without a throw, and the pins name their axis and scale', () => {
  for (const [kind] of KINDS) {
    const log = [];
    chart(pen(log), BOX, ROWS, ['#008cff', '#66aaff'], tones(VIEW), { kind, base: 3, at: 2, shown: () => 5 });
    expect([kind, log.length > 20]).toEqual([kind, true]);
  }
  const log = [];
  chart(pen(log), BOX, ROWS.slice(0, 1), ['#008cff'], tones(VIEW), { kind: 'pins', at: -1, shown: () => 5 });
  expect(said(log)).toContain('level L');
  expect(said(log)).toContain('log scale');
  expect(Object.keys(KIND)).toEqual(KINDS.map(([kind]) => kind));
});

test('an all-zero chart draws its one rule, a chart with a top draws two, and log pins force the log scale', () => {
  const rules = (rows, kind = 'steps') => {
    const log = [];
    chart(pen(log), BOX, rows, ['#008cff'], tones(VIEW), { kind, at: -1, shown: () => 9 });
    return log.filter(([key, text, x]) => key === 'fillText' && x < 90 && /^\d/.test(text)).map(([, text]) => text);
  };
  expect(rules([{ ...ROWS[0], terms: ['0', '0', '0', '0', '0'] }])).toEqual(['0']);
  expect(rules([{ ...ROWS[0], terms: ['8', '21', '40', '65'] }])).toEqual(['0', '65']);
  expect(rules([{ ...ROWS[0], terms: ['8', '21', '40', '65'] }], 'log')).toEqual(['1', '10', '100']);
});

test('a growth eases in new rows and only the new terms of a row, and a still shows every term at once', () => {
  const view = { ...VIEW, t: 0 };
  const grown = growth(view);
  const row = { key: 'a', terms: ['1', '2', '3', '4'] };
  expect(grown.shown(row)).toBe(0);
  view.t = 1000;
  expect(grown.shown(row)).toBe(4);
  expect(grown.busy([row])).toBe(false);
  row.terms.push('5', '6');
  expect(grown.shown(row)).toBe(4);
  expect(grown.busy([row])).toBe(true);
  view.t = 2000;
  expect(grown.shown(row)).toBe(6);
  row.terms.length = 3;
  expect(grown.shown(row)).toBe(3);
  expect(growth({ ...VIEW, still: true }).shown(row)).toBe(3);
});

test('the legend draws a row with cells as its side-3 design and a bare row as a swatch, and hands back its boxes', () => {
  expect(glyph(ROWS[0].cells)).toEqual({ slices: 1, width: 3 });
  expect(glyph(Array(27).fill(1))).toEqual({ slices: 3, width: 11 });
  expect([glyph(undefined), glyph([1, 0])]).toEqual([null, null]);
  const log = [];
  const { hits, height } = legend(pen(log), ROWS, ['#008cff', '#66aaff'], tones(VIEW), 16, 16, 760);
  expect(log.filter(([key]) => key === 'fillRect')).toHaveLength(10);
  expect(hits.map(({ i }) => i)).toEqual([0, 1]);
  expect(height).toBeGreaterThan(0);
  const one = [];
  legend(pen(one), [{ ...ROWS[0], hit: { id: 'A001018', name: 'Powers of 8', shift: 0, status: 'Proved' } }], ['#008cff'], tones(VIEW), 16, 16, 760);
  expect(said(one)).toContain('A001018 · Powers of 8');
});

test('the ribbon prints every term under its index and shortens long terms when the lines run out', () => {
  const log = [];
  const ctx = pen(log);
  const laid = ribbon(ctx, ROWS[0], tones(VIEW), 760);
  expect(laid.lines).toHaveLength(1);
  drawRibbon(ctx, laid, 16, 400, tones(VIEW), 2);
  expect(said(log)).toEqual(['1', '8', '2', '64', '3', '512', '4', '4096', '5', '32768']);
  const long = ribbon(ctx, { ...ROWS[0], terms: Array.from({ length: 32 }, (_, i) => '9'.repeat(i + 8)) }, tones(VIEW), 300, 4);
  expect(long.lines.length).toBeLessThanOrEqual(4);
  expect(long.lines[0][0].text).toBe('9.99e7');
});

test('text wraps by words and fits with an ellipsis, and a tap finds the slot under it', () => {
  const ctx = pen([]);
  expect(wrap(ctx, 'one two three four', 60)).toEqual(['one two', 'three', 'four']);
  expect(fit(ctx, 'abcdefghij', 49)).toBe('abcdef…');
  const slots = { x: 100, slot: 50, n: 4, y0: 0, y1: 200 };
  expect([slotAt(slots, 120, 50), slotAt(slots, 260, 50), slotAt(slots, 320, 50), slotAt(slots, 120, 300), slotAt(null, 0, 0)]).toEqual([0, 3, -1, -1, -1]);
});
