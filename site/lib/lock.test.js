import { expect, mock, test } from 'bun:test';

const scene = (id) => async () => ({ make: (canvas, view, value) => ({ id, value, draw: () => {} }) });

mock.module('apps:scenes', () => ({ scenes: { one: scene('one'), two: scene('two') } }));

const { lock } = await import('./lock.js');

test('a pick is the first saver, random, a saver or a saver with its value, an unknown saver falls back to random, and the lock names what it ran with its seed', async () => {
  Object.assign(globalThis, { window: new EventTarget(), document: Object.assign(new EventTarget(), { hidden: true }) });
  const seen = [];
  for (const pick of ['', 'random', 'two', 'two?seed=7&tone=red', 'gone?seed=3']) {
    const stop = await lock({ clientWidth: 4, clientHeight: 4 }, pick);
    const { seed, ...rest } = stop.scene.value;
    seen.push([pick, stop.pick, stop.scene.id, Number.isInteger(seed), rest, stop.as === `${stop.scene.id}?${new URLSearchParams(stop.scene.value)}`]);
    stop();
  }
  for (const name of ['window', 'document']) delete globalThis[name];
  const any = expect.stringMatching(/^(one|two)$/);
  expect(seen).toEqual([
    ['', '', 'one', true, {}, true],
    ['random', 'random', any, true, {}, true],
    ['two', 'two', 'two', true, {}, true],
    ['two?seed=7&tone=red', 'two?seed=7&tone=red', 'two', true, { tone: 'red' }, true],
    ['gone?seed=3', 'random', any, true, {}, true],
  ]);
});
