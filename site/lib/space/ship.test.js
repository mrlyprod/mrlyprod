import { expect, test } from 'bun:test';
import { rng } from '../scene.js';
import { look, project } from './camera.js';
import { gl } from './fake.js';
import { ship } from './ship.js';

const view = { rand: rng(7), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 1280, h: 720, dpr: 1, t: 0 };

const CAM = look([0, 0, 0], [0, 0, 1]);

const SIDEWAYS = [[0, 0, -1], [0, 1, 0], [1, 0, 0]];

const POSE = { at: [0, 0, 4], axes: SIDEWAYS, size: 0.5, fwd: [1, 0, 0], heat: 0.4, plume: 1 };

test('a ship glows by one additive fill inside a scissor box that holds the ship and its plume', () => {
  const fake = gl();
  fake.viewport(0, 0, 1280, 720);
  ship(fake, view).draw(CAM, POSE);
  const [x, y, w, h] = fake.log.find(([key]) => key === 'scissor').slice(1);
  const inside = (p) => p[0] > x && p[0] < x + w && 720 - p[1] > y && 720 - p[1] < y + h;
  const draw = fake.draws().at(-1);
  expect([fake.draws().length, [POSE.at, [-1.5, 0, 4]].every((p) => inside(project(view, CAM, p))), draw.uniforms.uHeat, fake.log.some(([key, a, b]) => key === 'blendFunc' && a === fake.ONE && b === fake.ONE)]).toEqual([1, true, 0.4, true]);
});

test('the nose heat is never cut by the ship box, so drawn under the hull only the hull hides it, while the plume stops at the box', () => {
  const fake = gl();
  ship(fake, view).draw(CAM, POSE);
  const { fs } = fake.draws().at(-1);
  expect([/HOT \* uHeat[^;]*spot\([^;]*OPEN\)/.test(fs), /uFire \* uPlume \* spot\([^;]*wall\)/.test(fs)]).toEqual([true, true]);
});

test('a ship with its engines off and a cold nose draws nothing', () => {
  const fake = gl();
  ship(fake, view).draw(CAM, { ...POSE, heat: 0, plume: 0 });
  expect(fake.draws()).toEqual([]);
});
