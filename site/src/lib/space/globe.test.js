import { expect, test } from 'bun:test';
import { NAMES, world } from './worlds.js';
import { globe } from './globe.js';

const paint = (params, side = 56) => {
  const log = [];
  const stop = { addColorStop: (...args) => log.push(['stop', ...args]) };
  const ctx = new Proxy(
    {},
    {
      get: (_, key) => (key.startsWith('create') ? () => stop : (...args) => log.push([key, ...args])),
      set: (_, key, value) => {
        log.push([key, value]);
        return true;
      },
    },
  );
  globe(ctx, side, side, params);
  return log;
};

test('every world paints a disc inside its square, and only a ringed world strokes an ellipse', () => {
  for (const name of NAMES) {
    const params = world(name, 3);
    const log = paint(params);
    const disc = log.find(([key]) => key === 'arc');
    expect([name, disc[1] - disc[3] >= 0, disc[1] + disc[3] <= 56, log.some(([key]) => key === 'ellipse')]).toEqual([name, true, true, Boolean(params.ring)]);
  }
});

test('a gas giant fills with one stop per band, and a rocky world with its sea first', () => {
  const gas = world('jupiter', 0);
  expect(paint(gas).filter(([key]) => key === 'stop').length).toBe(gas.bands.length + 3);
  const earth = world('earth', 0);
  const fills = paint(earth).filter(([key]) => key === 'fillStyle');
  expect(fills[0][1]).toMatch(/^rgba\(\d+,\d+,\d+,1\)$/);
  expect(Number(fills[0][1].match(/\d+/g)[2])).toBeGreaterThan(Number(fills[0][1].match(/\d+/g)[0]));
});
