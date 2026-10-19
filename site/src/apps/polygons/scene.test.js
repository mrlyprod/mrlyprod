import { expect, test } from 'bun:test';
import { rng } from '../../lib/rand.js';
import { CROSS, FIRST, SHAPES } from './engine.js';
import { make } from './scene.js';

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (target, key) => (key in target ? target[key] : (...args) => log.push([key, ...args.filter((one) => typeof one !== 'object')])),
      set: (target, key, value) => {
        if (key === 'fillStyle') log.push(['fillStyle', value]);
        target[key] = value;
        return true;
      },
    },
  );

function stage(opts = {}) {
  const shown = [];
  const drawn = [];
  const cache = { width: 0, height: 0, getContext: () => pen(drawn) };
  const canvas = { getContext: () => pen(shown), ownerDocument: { createElement: () => cache } };
  const view = { rand: rng(1), look: () => ({ paper: '#101010', accent: '#008cff' }), still: false, w: 320, h: 200, dpr: 1, t: 0 };
  const scene = make(canvas, view, { seed: 7, ...opts });
  const at = (t) => {
    view.t = t;
    scene.draw();
  };
  return { shown, drawn, at };
}

const fills = (log) => log.filter(([verb]) => verb === 'fill').length;

test('every shape paints the mosaic once on the first frame and shows it', () => {
  for (const shape of SHAPES) {
    const { shown, drawn, at } = stage({ shape });
    at(0);
    expect([shape, shown.filter(([verb]) => verb === 'drawImage').length, drawn.filter(([verb]) => verb === 'fillRect').length, fills(drawn) > 0]).toEqual([shape, 1, 1, true]);
  }
});

test('one seed and t paint one picture, waves and all', () => {
  const [one, two] = [stage(), stage()];
  for (const { at } of [one, two]) for (const t of [0, FIRST / 2, FIRST + CROSS]) at(t);
  expect([one.drawn.length > 0, one.drawn, one.shown]).toEqual([true, two.drawn, two.shown]);
});

test('a frame before the first wave is not shown again', () => {
  const { shown, at } = stage();
  at(0);
  at(FIRST / 2);
  expect(shown.filter(([verb]) => verb === 'drawImage').length).toBe(1);
});
