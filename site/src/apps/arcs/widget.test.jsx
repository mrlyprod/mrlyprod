import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { study } from './engine.js';
import { named, rows } from './widget.jsx';

const value = { seed: 1, base: 2, code: '', level: 5, look: 'all', width: 0, cells: 1, tint: 'orange', strand: 'blue' };

test('the readouts lead with the design and the loops, name the law at its level and say whether it meets the count', () => {
  const read = (over) => Object.fromEntries(rows(study(math, { ...value, ...over }).facts));
  expect(read({})).toEqual({ design: 'carpet 7', loops: '50', strands: '64 = 2 x 32, proved', side: '32', filled: '243 of 1024', law: 'L(n) = 3^(n-1) - 2^n + 1, n >= 1', 'L(5)': '50, the law meets the count' });
  expect(rows(study(math, { ...value, base: 3, code: '500', level: 2 }).facts).map(([name]) => name)).toEqual(['design', 'loops', 'strands', 'side', 'filled', 'law']);
  expect(read({ base: 3, code: '500', level: 2 }).law).toBe('no closed form proved for this code');
  expect(read({ code: '99' })).toEqual({ design: 'code 99', fault: 'code 99 out of range for dimension 2 base 2 (0..15).' });
  expect(rows(null)).toEqual([]);
});

test('the named designs carry their algebraic normal form at base 2 and their word at the other bases', () => {
  expect(named(math, 2)[0]).toEqual(['7', 'carpet · 1+xy']);
  expect(named(math, 3)[0]).toEqual(['495', 'carpet']);
  expect(named(math, 5)).toHaveLength(math.name.words(2, 5).length);
});
