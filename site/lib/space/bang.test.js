import { expect, test } from 'bun:test';
import { rng } from '../scene.js';
import { PILOTS, canon, cells, classes, codes, doors, gate, name, pickCode, route, teach } from './bang.js';

const at = (n, x, y, z) => (z * n + y) * n + x;

const DIVE = [15, 51, 63, 85, 95, 119, 127, 255];

test('cells keep a cell by the bit of its parities, x the slowest', () => {
  const sponge = cells(23, 3);
  expect([[0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1], [1, 1, 0], [0, 1, 1], [1, 1, 1], [2, 2, 2]].map(([x, y, z]) => sponge[at(3, x, y, z)])).toEqual([1, 1, 1, 1, 0, 0, 0, 1]);
  expect([cells(16, 3)[at(3, 1, 0, 0)], cells(16, 3)[at(3, 0, 0, 1)], cells(2, 3)[at(3, 0, 0, 1)], cells(2, 3)[at(3, 1, 0, 0)]]).toEqual([1, 0, 1, 0]);
  expect(cells(23, 3).reduce((a, b) => a + b, 0)).toBe(20);
});

test('the sponge passes the gate at n 3; the empty, the single cell and the full cube fail', () => {
  expect([23, 0, 128, 255].map((code) => gate(code, 3))).toEqual([true, false, false, false]);
});

test('every pruned exit has an onward exit', () => {
  for (const n of [3, 5]) {
    for (let code = 0; code < 256; code++) {
      const table = doors(cells(code, n), n);
      for (const one of table.filter(Boolean)) for (const exit of one.exits) expect([code, n, table[exit.dir]?.exits.length > 0]).toEqual([code, n, true]);
    }
  }
});

test('the 48 signed permutations of the parity bits cut the 256 codes into 22 classes', () => {
  expect(new Set(Array.from({ length: 256 }, (_, code) => canon(code))).size).toBe(22);
  expect(classes().length).toBe(21);
  expect([canon(0), canon(255), canon(232), classes()[0], classes().includes(0)]).toEqual([0, 255, 23, 1, false]);
});

test('a code reads its own name, else its class mate in list order, else its class', () => {
  const mate = Array.from({ length: 256 }, (_, code) => code).find((code) => canon(code) === 23 && ![23, 232].includes(code));
  expect([name(232), name(3), name(17), name(5), name(mate), name(60)]).toEqual(['net', 'ztree', 'xtree', 'ytree', 'carpet', `class ${canon(60)}`]);
});

test('teach gives the bits, the kept cells, the fill, the dimension and the class', () => {
  const sponge = teach(23, 3);
  expect([sponge.bits, sponge.kept, sponge.fill, sponge.class]).toEqual(['00010111', 20, 20 / 27, 23]);
  expect(Math.abs(sponge.dimension - 2.7268)).toBeLessThan(1e-3);
});

test('codes picks all, the flyable, the named classes, the family, and the gated list with no pick', () => {
  expect([codes(3, 'all').length, codes(3, 'flyable').length, codes(5, 'flyable').length, codes(3, 'named'), codes(3, 'family'), codes(3).length]).toEqual([255, 247, 247, classes(), [23, 232, 22, 129, 17, 5, 3], 61]);
  expect(codes(3, 'all').filter((code) => !codes(3, 'flyable').includes(code))).toEqual(DIVE);
  expect(codes(5).filter((code) => code & 1)).toEqual([]);
});

test('pickCode keeps a set code and draws -1 from the pick by the seed', () => {
  const one = pickCode({ code: -1, n: 3, pick: 'named' }, 5);
  expect([classes().includes(one), pickCode({ code: -1, n: 3, pick: 'named' }, 5), pickCode({ code: 129, n: 3, pick: 'named' }, 5), codes(3).includes(pickCode({ code: -1, n: 3 }, 5))]).toEqual([true, one, 129, true]);
});

test('every code flies 120 s at n 3, the dive codes and 128 at n 5, with a finite camera within n of the frame centre', () => {
  for (const [n, list] of [[3, codes(3, 'all')], [5, [...DIVE, 128]]]) {
    for (const code of list) {
      const path = route({ code, n }, code);
      let ok = true;
      for (let tau = 0; tau <= 120; tau += 1) {
        const state = path.at(tau);
        ok &&= [...state.pos, ...state.fwd, ...state.up, state.roll, state.scale].every(Number.isFinite) && Math.hypot(...state.pos) <= n;
      }
      expect([code, n, ok]).toEqual([code, n, true]);
    }
  }
});

test('the eight codes with no way in skim outside over a face, every other code flies inside', () => {
  for (const n of [3, 5]) {
    const outside = codes(n, 'all').filter((code) => route({ code, n }, 7).at(10).outside);
    expect(outside).toEqual(DIVE);
  }
  expect(route({ code: 255, n: 3 }, 7).at(10).mode).toBe('skim');
});

test('the mode is read on the path flown: the sponge a corridor, dust and rods pillars, walls with windows a trench', () => {
  const most = (code) => {
    const path = route({ code, n: 3 }, 7);
    const count = {};
    for (let tau = 0; tau <= 60; tau += 0.5) count[path.at(tau).mode] = (count[path.at(tau).mode] ?? 0) + 1;
    return Object.entries(count).sort((a, b) => b[1] - a[1])[0][0];
  };
  expect([23, 2, 1, 3, 7].map(most)).toEqual(['corridor', 'pillars', 'pillars', 'pillars', 'trench']);
});

test('the sponge camera keeps 0.4 cells from every kept cell at its own level', () => {
  const n = 3;
  const path = route({ code: 23, n }, 7);
  const sponge = cells(23, n);
  const solid = (level, c, depth = 0) => {
    if (c.every((v) => v >= 0 && v < n)) return sponge[at(n, ...c)];
    if (level <= 0) return 0;
    const up = path.toFrame({ j: level, p: c.map((v) => v + 0.5 - n / 2) }, level - 1).map((v) => Math.floor(v + n / 2));
    return depth < 3 && solid(level - 1, up, depth + 1) ? sponge[at(n, ...c.map((v) => ((v % n) + n) % n))] : 0;
  };
  let near = Infinity;
  for (let tau = 0; tau <= 30; tau += 0.05) {
    const state = path.at(tau);
    const lev = state.frame - Math.log(state.scale) / Math.log(n);
    const level = Math.ceil(lev - 1e-6);
    const q = path.toFrame({ j: state.frame, p: state.pos }, level).map((v) => v + n / 2);
    for (let i = 0; i < 27; i++) {
      const c = q.map((v, k) => Math.floor(v) + (Math.floor(i / 3 ** k) % 3) - 1);
      if (solid(level, c)) near = Math.min(near, Math.hypot(...q.map((v, k) => Math.max(c[k] - v, 0, v - c[k] - 1))) * n ** (lev - level));
    }
  }
  expect(near).toBeGreaterThan(0.4);
});

test('before the entry face there is no camera; on the straight after it the speed holds 1.15 cells a second within 1%', () => {
  const path = route({ code: 24, n: 5 }, 2);
  let tau = -10;
  while (!Number.isFinite(path.at(tau).pos[0])) tau += 0.05;
  const speeds = [];
  for (let prev = path.at(tau), t = tau + 0.05; t < tau + 1.2; t += 0.05) {
    const state = path.at(t);
    speeds.push(Math.hypot(...state.pos.map((v, k) => v - prev.pos[k])) / state.scale / 0.05);
    prev = state;
  }
  expect(speeds.every((v) => Math.abs(v / 1.15 - 1) < 0.01)).toBe(true);
});

test('across a rebase toFrame carries the camera on without a jump', () => {
  const path = route({ code: 23, n: 3 }, 7);
  let prev = path.at(0);
  let rebases = 0;
  for (let tau = 0.02; tau <= 30; tau += 0.02) {
    const state = path.at(tau);
    if (state.frame !== prev.frame) {
      rebases++;
      const carried = path.toFrame({ j: prev.frame, p: prev.pos }, state.frame);
      expect(Math.hypot(...carried.map((v, k) => v - state.pos[k])) / state.scale).toBeLessThan(0.05);
    }
    prev = state;
  }
  expect(rebases).toBeGreaterThan(3);
});

test('the route is a pure function of seed and pilot: play, a jump and any order of taus agree', () => {
  const read = (state) => [state.level, state.frame, state.mode, ...state.pos, ...state.fwd, ...state.up, state.roll];
  for (const pilot of Object.keys(PILOTS)) {
    const make = () => route({ code: 23, n: 3 }, 9, { pilot });
    const taus = Array.from({ length: 50 }, (_, i) => 3 + i * 2.3);
    const played = make();
    for (let tau = 0; tau <= taus.at(-1); tau += 1 / 60) played.at(tau);
    const again = make();
    const want = new Map(taus.map((tau) => [tau, read(again.at(tau))]));
    const jumped = make();
    const rand = rng(4);
    for (const tau of [...taus].sort(() => rand() - 0.5)) expect([pilot, tau, ...read(jumped.at(tau))]).toEqual([pilot, tau, ...want.get(tau)]);
    expect(read(played.at(taus.at(-1)))).toEqual(want.get(taus.at(-1)));
  }
  expect(route({ code: 23, n: 3 }, 10).at(120).pos).not.toEqual(route({ code: 23, n: 3 }, 9).at(120).pos);
});

test('path samples the camera ahead: sample k is the camera at tau + k dt carried into the frame at tau, across rebases', () => {
  const span = 8;
  const ahead = route({ code: 23, n: 3 }, 7);
  const flown = route({ code: 23, n: 3 }, 7);
  let crossed = 0;
  for (const tau of [5, 17.3, 40]) {
    const line = ahead.path(tau, span);
    const here = flown.at(tau);
    const dt = span / (line.length - 1);
    line.forEach((p, k) => {
      const there = flown.at(tau + k * dt);
      if (there.frame !== here.frame) crossed++;
      const want = flown.toFrame({ j: there.frame, p: there.pos }, here.frame);
      expect([tau, k, Math.max(...p.map((v, i) => Math.abs(v - want[i]))) < 1e-6]).toEqual([tau, k, true]);
    });
  }
  expect(crossed).toBeGreaterThan(0);
});

test('the fighter and the coaster fly the sponge and a trench 120 s with a finite camera and rail', () => {
  for (const pilot of ['fighter', 'coaster']) {
    for (const code of [23, 7]) {
      const path = route({ code, n: 3 }, 7, { pilot });
      let ok = true;
      for (let tau = 0; tau <= 120; tau += 0.25) {
        const state = path.at(tau);
        ok &&= [...state.pos, ...state.fwd, ...state.up, state.roll].every(Number.isFinite) && path.path(tau, 8).flat().every(Number.isFinite);
      }
      expect([pilot, code, ok]).toEqual([pilot, code, true]);
    }
  }
});

test('near marks the 5x5x5 blocks around the frame block, and edge counts the blocks from it to the root faces', () => {
  const path = route({ code: 23, n: 3 }, 7);
  const bit = (words, q) => {
    const i = (q[0] + 2) * 25 + (q[1] + 2) * 5 + q[2] + 2;
    return (words[i >> 5] >> (i & 31)) & 1;
  };
  for (const tau of [0, 12, 30]) {
    const { near, edge, frame } = path.at(tau);
    const [low, high] = edge;
    const outside = [[-2, 0, 0], [2, 0, 0], [0, -2, 0], [0, 2, 0], [0, 0, -2], [0, 0, 2]].filter((q) => q.some((v, i) => v < -low[i] || v > high[i]));
    expect([tau, low.map((v, i) => v + high[i]), bit(near, [0, 0, 0]), outside.map((q) => bit(near, q))]).toEqual([tau, [3 ** frame - 1, 3 ** frame - 1, 3 ** frame - 1], 1, outside.map(() => 0)]);
  }
});

test('the fighter barrel-rolls a full turn on a straight, the steady pilot never', () => {
  const most = (pilot) => {
    const path = route({ code: 23, n: 3 }, 7, { pilot });
    return Math.max(...Array.from({ length: 1200 }, (_, i) => Math.abs(path.at(i * 0.05).roll)));
  };
  expect([most('fighter') > 6, most('steady') < 0.5]).toEqual([true, true]);
});
