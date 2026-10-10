import { expect, test } from 'bun:test';
import { look } from './camera.js';
import { gl } from './fake.js';
import { trails } from './trails.js';

const view = (t) => ({ look: () => ({ paper: '#000000', accent: '#008cff' }), w: 1280, h: 720, dpr: 1, t });

const CAM = look([0, 0, 0], [0, 0, 1]);

const LINE = Array.from({ length: 11 }, (_, i) => [0.1, -0.2, 1 + i]);

const setup = (t) => {
  const fake = gl();
  fake.viewport(0, 0, 1280, 720);
  const at = view(t);
  return { fake, at, lines: trails(fake, at) };
};

const sent = (fake, count) => Array.from({ length: count * 2 }, (_, i) => fake.log.filter(([key]) => key === 'bufferSubData').at(-1)[3].slice(i * 4, i * 4 + 4));

const head = (fake, count) => sent(fake, count).reduce((top, v) => (v[3] > top[3] ? v : top))[2];

test('the pulse head runs down the rail from its first point at 6 cells a second and starts again every 1.5 s', () => {
  const { fake, at, lines } = setup(0);
  const heads = [250, 500, 1750].map((t) => {
    at.t = t;
    return +head(fake, lines.draw([{ points: LINE, born: 0 }], CAM)).toFixed(2);
  });
  const near = Math.hypot(0.1, 0.2, 1);
  expect(heads.map((d) => +Math.sqrt(d * d - near * near + 1).toFixed(1))).toEqual([2.5, 4, 2.5]);
});

test('step sets the cells between points, so the head keeps 6 cells a second on a denser rail', () => {
  const { fake, lines } = setup(250);
  const dense = Array.from({ length: 21 }, (_, i) => [0.1, -0.2, 1 + i / 2]);
  const d = head(fake, lines.draw([{ points: dense, born: 0, step: 0.5 }], CAM));
  expect(+Math.sqrt(d * d - 0.05).toFixed(1)).toBe(2.5);
});

test('the rail adds its light in the accent', () => {
  const { fake, lines } = setup(250);
  lines.draw([{ points: LINE, born: 0 }], CAM);
  expect(fake.log.some(([key, a, b]) => key === 'blendFunc' && a === fake.ONE && b === fake.ONE)).toBe(true);
});

test('a point behind the eye is cut at the near plane, a NaN point drops its segments, and the march depth hides what lies past a wall', () => {
  const { fake, lines } = setup(2000);
  expect(lines.draw([{ points: [[0, 0, -1], [0.2, 0, 1], [NaN, 0, 2]], born: 0 }], CAM, 1, { id: 99 })).toBe(1);
  const [[ax, ay, az]] = sent(fake, 1);
  const draw = fake.draws().at(-1);
  expect([Number.isFinite(ax) && Number.isFinite(ay), az > 0 && az < 0.2, draw.uniforms.uHide, draw.units[0]]).toEqual([true, true, 1, 99]);
});
