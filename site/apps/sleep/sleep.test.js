import { expect, test } from 'bun:test';
import { rng } from '../../lib/scene.js';
import { bounce, make } from './scene.js';

test('the sleeper clamps and flips at both walls', () => {
  expect(bounce(-2, -4, 100)).toEqual({ p: 0, v: 4, hit: true });
  expect(bounce(104, 4, 100)).toEqual({ p: 100, v: -4, hit: true });
  expect(bounce(50, 4, 100)).toEqual({ p: 50, v: 4, hit: false });
});

test('the sleeper covers the same ground in a second at 60 Hz and at 120 Hz, on whole cells', () => {
  const corner = (hz) => {
    const rects = [];
    const ctx = new Proxy({}, { get: (_, key) => (key === 'rect' ? (...args) => rects.push(args) : () => {}), set: () => true });
    const view = { rand: rng(3), look: () => ({ accent: '#000' }), still: false, w: 4000, h: 3000, dpr: 2, t: 0 };
    const scene = make({ getContext: () => ctx }, view, { speed: 240, size: 0.1 });
    for (let n = 1; n <= hz; n++) {
      view.t = (n * 1000) / hz;
      rects.length = 0;
      scene.draw();
    }
    return { at: rects[0].slice(0, 2), whole: rects.every((one) => one.every(Number.isInteger)) };
  };
  const slow = corner(60);
  expect(corner(120)).toEqual(slow);
  expect(slow.whole).toBe(true);
});
