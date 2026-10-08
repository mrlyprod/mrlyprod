import { expect, mock, test } from 'bun:test';

const scene = (id) => async () => ({ make: (canvas, view, value) => ({ id, value, draw: () => {} }) });

mock.module('apps:scenes', () => ({ scenes: { one: scene('one'), two: scene('two') } }));

const { lock } = await import('./lock.js');

test('a pick is none, random, a saver or a saver with its value, and an unknown saver falls back to random', async () => {
  Object.assign(globalThis, { window: new EventTarget(), document: Object.assign(new EventTarget(), { hidden: true }) });
  const seen = [];
  for (const pick of ['', 'random', 'two', 'two?seed=7&tone=red', 'gone?seed=3']) {
    const stop = await lock({ clientWidth: 4, clientHeight: 4 }, pick);
    seen.push([pick, stop.pick, stop.scene?.id ?? '', stop.scene?.value ?? null]);
    stop();
  }
  for (const name of ['window', 'document']) delete globalThis[name];
  const any = expect.stringMatching(/^(one|two)$/);
  expect(seen).toEqual([
    ['', '', '', null],
    ['random', 'random', any, {}],
    ['two', 'two', 'two', {}],
    ['two?seed=7&tone=red', 'two?seed=7&tone=red', 'two', { seed: 7, tone: 'red' }],
    ['gone?seed=3', 'random', any, {}],
  ]);
});
