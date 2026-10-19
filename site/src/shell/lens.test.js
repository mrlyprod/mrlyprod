import { expect, test } from 'bun:test';
import { lens } from './lens.js';

const W = 48;

const sizes = [W, 0, ...Array(8).fill(W), 0, W, W];

const lefts = sizes.reduce((out, w, i) => [...out, i ? out[i - 1] + (sizes[i - 1] || 9) + 4 : 0], []);

const REST = sizes.map((w, i) => ({ c: lefts[i] + (w || 9) / 2, dx: 0, w }));

const under = (items, x, m = items.map(() => 1), dx = items.map(() => 0)) => items.findIndex(({ c, w }, i) => w && Math.abs(x - c - dx[i]) <= (w * m[i]) / 2);

test('the tile under a still cursor stays under it and is the largest, and the grown layout gives the same lens back', () => {
  for (let x = -9.5; x < lefts.at(-1) + W + 9; x += 3) {
    const one = lens(REST, x);
    const at = under(REST, x, one.m, one.dx);
    expect(at).toBe(under(REST, x));
    if (at >= 0) expect(one.m[at]).toBe(Math.max(...one.m));
    expect(lens(REST.map((item, i) => ({ ...item, c: item.c + one.dx[i], dx: one.dx[i] })), x)).toEqual(one);
  }
});
