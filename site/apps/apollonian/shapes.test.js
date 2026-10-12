import { expect, test } from 'bun:test';
import { circles, clip, labels, rules } from './shapes.js';

test('circles, labels, rules and a clip compose to svg bodies and stay empty with nothing to draw', () => {
  expect(circles([[1, 2, 3.456]], { fill: '#abc' })).toBe('<g fill="#abc" stroke="none" stroke-width="1"><circle cx="1" cy="2" r="3.46"/></g>');
  expect(circles([[1, 2, 3]], { stroke: '#abc', width: 1.25 })).toBe('<g fill="none" stroke="#abc" stroke-width="1.25"><circle cx="1" cy="2" r="3"/></g>');
  expect(labels([[10, 20, 12, '8']], '#abc', 'Mono')).toBe('<g fill="#abc" font-family="Mono" text-anchor="middle" dominant-baseline="central"><text x="10" y="20" font-size="12">8</text></g>');
  expect(rules([[0, 1, 2, 3]], '#abc', 2)).toBe('<g stroke="#abc" stroke-width="2"><line x1="0" y1="1" x2="2" y2="3"/></g>');
  expect(clip('p', [0, 1, 2, 3], '<g/>')).toBe('<clipPath id="p"><rect x="0" y="1" width="2" height="3"/></clipPath><g clip-path="url(#p)"><g/></g>');
  expect([circles([], {}), labels([], '#abc', 'Mono'), rules([], '#abc', 1), clip('p', [0, 0, 1, 1], '')]).toEqual(['', '', '', '']);
});
