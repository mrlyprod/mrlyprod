import { expect, test } from 'bun:test';
import { grid, lines, marks, num, path, rects, runs, sheet } from './svg.js';

const carpet = { shape: [3, 3], types: Uint8Array.from([1, 1, 1, 1, 0, 1, 1, 1, 1]) };

test('num rounds to two places, or to the places asked', () => {
  expect([num(20.7846), num(20.7846, 3), num(3), num(0.004)]).toEqual(['20.78', '20.785', '3', '0']);
});

test('a sheet is one svg of the size with a view box of the same size unless given', () => {
  expect(sheet(200, 100.504, '<g/>')).toBe('<svg xmlns="http://www.w3.org/2000/svg" width="200" height="100.5" viewBox="0 0 200 100.5"><g/></svg>');
  expect(sheet(30, 20, '', '0 0 3 2')).toContain('viewBox="0 0 3 2"');
});

test('runs are the filled stretches of each row as row, start and length', () => {
  expect(runs(carpet)).toEqual([[0, 0, 3], [1, 0, 1], [1, 2, 1], [2, 0, 3]]);
  expect(runs({ shape: [1, 2], types: Uint8Array.from([0, 0]) })).toEqual([]);
});

test('marks map row-major cell indices to columns and rows', () => {
  expect(marks([0, 4, 5], 3)).toEqual([[0, 0], [1, 1], [2, 1]]);
  expect(marks([], 0)).toEqual([]);
});

test('a grid lights the listed indices of a rows by cols cell', () => {
  expect(grid(2, 3, [1, 5])).toEqual({ shape: [2, 3], types: Uint8Array.from([0, 1, 0, 0, 0, 1]) });
});

test('rects write one crisp rect per run over an optional ground, unit px a cell', () => {
  const text = rects(carpet, { fill: '#008cff', ground: '#000', unit: 10 });
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="30" height="30" viewBox="0 0 30 30">')).toBe(true);
  expect(text.match(/<rect /g)).toHaveLength(5);
  expect(text).toContain('<rect width="30" height="30" fill="#000"/>');
  expect(text).toContain('<g fill="#008cff" shape-rendering="crispEdges">');
  expect(text).toContain('<rect x="20" y="10" width="10" height="10"/>');
  expect(rects(carpet, { fill: '#fff', unit: 2.5 })).not.toContain('fill="#000"');
  expect(rects(carpet, { fill: '#fff', unit: 2.5 })).toContain('<rect x="5" y="2.5" width="2.5" height="2.5"/>');
});

test('a path moves to the first point and lines to the rest, closed on request', () => {
  expect(path([1.239, 2.235, 3, 4])).toBe('M1.24 2.24L3 4');
  expect(path([0, 0, 10, 0, 10, 10], true)).toBe('M0 0L10 0L10 10Z');
  expect(path([])).toBe('');
});

test('lines write one rounded path per stroke, dashed where asked, and skip an empty one', () => {
  const text = lines({ width: 200, height: 100.5, strokes: [{ points: [0, 0, 10, 0, 10, 10], color: 'rgb(1, 2, 3)', width: 1.25, dash: [4, 6], closed: true }, { points: new Float32Array([1.004, 2, 3, 4]), color: '#fff' }, { points: [] }] });
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="200" height="100.5" viewBox="0 0 200 100.5">')).toBe(true);
  expect(text.match(/<path /g)).toHaveLength(2);
  expect(text).toContain('<path d="M0 0L10 0L10 10Z" fill="none" stroke="rgb(1, 2, 3)" stroke-width="1.25" stroke-linecap="round" stroke-linejoin="round" stroke-dasharray="4 6"/>');
  expect(text).toContain('<path d="M1 2L3 4" fill="none" stroke="#fff" stroke-width="1"');
});
