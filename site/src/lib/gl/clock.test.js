import { expect, test } from 'bun:test';
import { clock, flown } from './clock.js';

const PHASES = [['wind', 300], ['stretch', 900], ['tunnel', Infinity], ['decay', 900]];

test('phases follow each other, and an Infinity phase holds until end()', () => {
  const c = clock(PHASES);
  expect(c.at(0).name).toBeNull();
  c.start(1000);
  expect(c.at(1150)).toEqual({ name: 'wind', k: 0.5, since: 150, left: 150 });
  expect(c.at(1300).name).toBe('stretch');
  expect(c.at(1300 + 900 + 1e6)).toEqual({ name: 'tunnel', k: 0, since: 1e6, left: Infinity });
  c.end(5000);
  expect(c.at(4999).name).toBe('tunnel');
  expect(c.at(5450)).toEqual({ name: 'decay', k: 0.5, since: 450, left: 450 });
  expect(c.at(5900)).toEqual({ name: null, k: 1, since: 0, left: 0 });
});

test('flown equals a fine numeric sum of the speed curve', () => {
  const speeds = { wind: [0.1, -0.04, 1], stretch: [0.015, 14, 2.5], tunnel: 6, decay: [6, 0.015, 0.5] };
  const c = clock(PHASES);
  c.start(1000);
  c.end(4000);
  const speed = (t) => {
    const { name, k } = c.at(t);
    const v = speeds[name] ?? 0;
    return typeof v === 'number' ? v : v[0] + (v[1] - v[0]) * k ** v[2];
  };
  const dt = 0.01;
  let sum = 0;
  let t = 1000;
  for (const end of [1150, 1700, 3000, 4000, 4600, 6000]) {
    for (; t < end - dt / 2; t += dt) sum += (speed(t + dt / 2) * dt) / 1000;
    expect(flown(c, speeds, end)).toBeCloseTo(sum, 5);
  }
  expect(flown(c, speeds, 500)).toBe(0);
});
