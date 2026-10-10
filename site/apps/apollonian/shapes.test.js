import { expect, test } from 'bun:test';
import { box, discs, labels, rings, rules } from './shapes.js';

test('discs, rings, labels, rules and a box compose to svg bodies and stay empty with nothing to draw', () => {
  expect(discs([[1, 2, 3.456]], '#abc')).toBe('<g fill="#abc"><circle cx="1" cy="2" r="3.46"/></g>');
  expect(rings([[1, 2, 3]], '#abc', 1.25)).toBe('<g fill="none" stroke="#abc" stroke-width="1.25"><circle cx="1" cy="2" r="3"/></g>');
  expect(labels([[10, 20, 12, '8']], '#abc', 'Mono')).toBe('<g fill="#abc" font-family="Mono" text-anchor="middle" dominant-baseline="central"><text x="10" y="20" font-size="12">8</text></g>');
  expect(rules([[5, 10, 5, 30, 0.5]], '#abc', 1)).toBe('<g stroke="#abc" stroke-width="1"><line x1="5" y1="10" x2="5" y2="30" stroke-opacity="0.5"/></g>');
  expect(rules([[0, 1, 2, 3]], '#abc', 2)).toBe('<g stroke="#abc" stroke-width="2"><line x1="0" y1="1" x2="2" y2="3"/></g>');
  expect(box(0, 1, 2, 3, '#abc')).toBe('<rect x="0" y="1" width="2" height="3" fill="#abc"/>');
  expect([discs([], '#abc'), rings([], '#abc', 1), labels([], '#abc', 'Mono'), rules([], '#abc', 1)]).toEqual(['', '', '', '']);
});
