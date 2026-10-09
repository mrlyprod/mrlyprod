import { expect, test } from 'bun:test';
import { rng } from '../scene.js';
import { cells, codes, doors, gate, ride, route, totem } from './bang.js';
import { look, project } from './camera.js';
import { gl } from './fake.js';

const at = (n, x, y, z) => (z * n + y) * n + x;

test('cells keep a cell by the bit of its parities, x the slowest', () => {
  const sponge = cells(23, 3);
  expect([[0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1], [1, 1, 0], [0, 1, 1], [1, 1, 1], [2, 2, 2]].map(([x, y, z]) => sponge[at(3, x, y, z)])).toEqual([1, 1, 1, 1, 0, 0, 0, 1]);
  expect([cells(16, 3)[at(3, 1, 0, 0)], cells(16, 3)[at(3, 0, 0, 1)], cells(2, 3)[at(3, 0, 0, 1)], cells(2, 3)[at(3, 1, 0, 0)]]).toEqual([1, 0, 1, 0]);
  expect(cells(23, 3).reduce((a, b) => a + b, 0)).toBe(20);
});

test('the sponge passes the gate at n 3; the empty, the single cell and the full cube fail', () => {
  expect([23, 0, 128, 255].map((code) => gate(code, 3))).toEqual([true, false, false, false]);
});

test('codes holds the gated codes of each n, and n 5 shuts every code with bit 0 set', () => {
  expect(codes(3).length).toBeGreaterThan(0);
  expect(codes(5).length).toBeGreaterThan(0);
  expect(codes(3)).toContain(23);
  expect(codes(5).filter((code) => code & 1)).toEqual([]);
});

test('every pruned exit has an onward exit', () => {
  for (const n of [3, 5]) {
    for (let code = 0; code < 256; code++) {
      const table = doors(cells(code, n), n);
      for (const one of table.filter(Boolean)) for (const exit of one.exits) expect([code, n, table[exit.dir]?.exits.length > 0]).toEqual([code, n, true]);
    }
  }
});

const dive = (path, levels, each = () => {}) => {
  let tau = 0;
  for (let state = path.at(tau); state.level < levels; state = path.at(tau)) {
    each(state);
    tau += 0.05;
  }
  return tau;
};

test('every gated code flies 40 levels with no dead end and the camera within n of the frame centre, the sponge 300', () => {
  const runs = [...codes(3).map((code) => [code, 3, 40]), ...codes(5).map((code) => [code, 5, 40]), [23, 3, 300]];
  for (const [code, n, levels] of runs) {
    let far = 0;
    dive(route({ code, n }, code), levels, (state) => {
      far = Math.max(far, Math.hypot(...state.pos));
    });
    expect([code, n, far <= n]).toEqual([code, n, true]);
  }
});

test('the route is a pure function of seed and choices, so a jump lands where play would', () => {
  const played = route({ code: 23, n: 3 }, 9);
  for (let tau = 0; tau < 3; tau += 1 / 60) played.at(tau);
  played.at(3);
  played.choose(1);
  for (let tau = 3; tau < 120; tau += 1 / 60) played.at(tau);
  const jumped = route({ code: 23, n: 3 }, 9);
  jumped.at(3);
  jumped.choose(1);
  const [a, b] = [played.at(120), jumped.at(120)];
  expect(b.level).toBe(a.level);
  b.pos.forEach((v, i) => expect(v).toBeCloseTo(a.pos[i], 9));
  expect(route({ code: 23, n: 3 }, 10).at(120).pos).not.toEqual(a.pos);
});

const view = { rand: rng(7), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 1280, h: 720, dpr: 1, t: 0 };

const AHEAD = look([0, 0, 0], [0, 0, 1]);

test('a totem of -1 or an empty code builds nothing', () => {
  const fake = gl();
  expect([totem(fake, view, { code: -1 }), totem(fake, view, { code: 0 })]).toEqual([null, null]);
  expect(fake.log).toEqual([]);
});

test('the totem stretches by 1 + 0.6 (e^(11 x) - 1) through the jump and rides whole in cruise and the tunnel', () => {
  const long = (name, k) => ride({ at: () => ({ name, k }), flow: () => ({ flicker: 0 }) }, AHEAD)[1];
  expect(long('stretch', 0)).toBe(1);
  expect(long('stretch', 0.5)).toBeCloseTo(1 + 0.6 * (Math.exp(11 * 0.3) - 1), 9);
  expect(long('pile', 1)).toBeCloseTo(1 + 0.6 * (Math.exp(11) - 1), 6);
  expect([long('cruise', 0), long('tunnel', 0.5), long('flash', 0.5), long('decay', 1)]).toEqual([1, 1, 1, 1]);
});

test('the totem draws once inside its scissor box, which holds its projection and reaches the vanishing point when stretched', () => {
  const fake = gl();
  fake.viewport(0, 0, 1280, 720);
  const one = totem(fake, view, { code: 23, seed: 7 });
  const box = (stretch) => {
    fake.log.length = 0;
    one.draw(AHEAD, undefined, stretch);
    return fake.log.find(([key]) => key === 'scissor').slice(1);
  };
  const [cx, cy] = project(view, AHEAD, [0, -0.35, 1.6]);
  const [x, y, w, h] = box(1);
  expect([x < cx && cx < x + w, y < 720 - cy && 720 - cy < y + h, w < 1280 / 5, h < 720 / 4]).toEqual([true, true, true, true]);
  const [, low, , tall] = box(1e4);
  expect([low <= y, Math.abs(low + tall - 360) < 4]).toEqual([true, true]);
  expect(fake.log.filter(([key]) => key === 'drawArrays').length).toBe(1);
});
