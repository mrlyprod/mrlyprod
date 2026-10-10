import { expect, test } from 'bun:test';
import * as life from '../../../pkgs/mrlyjs/life.js';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, describe, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { LINE } from './engine.js';
import { HOLD, SPEC, make } from './scene.js';

life.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/life/mrlyjs_life_bg.wasm', import.meta.url)).arrayBuffer() });
math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args]),
      set: (_, key, value) => {
        log.push([key, value]);
        return true;
      },
    },
  );

const open = (value, { still = false, w = 640, h = 480 } = {}) => {
  const log = [];
  const told = [];
  const live = { current: { ...defaults(SPEC), seed: 7, ...value } };
  const view = { rand: rng(7), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 1, t: 0 };
  const canvas = { getContext: () => pen(log) };
  const scene = make(canvas, view, { ...live.current, life, math, live, onStat: (stat) => told.push(stat) });
  const at = (t) => {
    log.length = 0;
    view.t = t;
    scene.draw();
  };
  return { log, told, scene, view, live, at, last: () => told[told.length - 1] };
};

const shown = (value) => describe(SPEC, { ...defaults(SPEC), ...value }).flatMap(({ rows }) => rows.map((row) => row.key));

test('a knob dead in a mode or under a start hides through its when', () => {
  expect(shown({ mode: 'life', from: 'noise' })).toEqual(['mode', 'born', 'stay', 'code', 'number', 'level', 'from', 'density', 'wrap', 'speed']);
  expect(shown({ mode: 'life', from: 'glider' })).toEqual(['mode', 'born', 'stay', 'code', 'number', 'level', 'from', 'wrap', 'speed']);
  expect(shown({ mode: 'wolfram', from: 'one' })).toEqual(['mode', 'rule', 'from', 'wrap', 'speed']);
  expect(shown({ mode: 'wolfram', from: 'design' })).toEqual(['mode', 'rule', 'code', 'number', 'from', 'wrap', 'speed']);
});

test('the Level pick offers the levels whose mask fits the budget, and tidy drops one that does not', () => {
  const row = SPEC.find((one) => one.key === 'level');
  expect([3, 5, 7, 9].map((number) => row.options({ number }).map(([level]) => level))).toEqual([[1, 2, 3], [1, 2], [1], [1]]);
  expect([tidy(SPEC, { number: 3, level: 3 }).level, tidy(SPEC, { number: 5, level: 3 }).level, tidy(SPEC, { number: 5, level: 2 }).level]).toEqual([3, 1, 2]);
});

test('play steps by t and speed, a jump lands where play would, and a draw with no new generation draws nothing', () => {
  const run = open({ speed: 10 });
  expect(run.last().gen).toBe(0);
  run.at(1000);
  expect([run.last().gen, run.log.some(([verb]) => verb === 'rect')]).toEqual([10, true]);
  run.at(1050);
  expect([run.last().gen, run.log.length]).toEqual([10, 0]);
  run.at(3000);
  expect(run.last().gen).toBe(30);
});

test('a jump past the budget of steps a frame rebases, so play never chases a lost second', () => {
  const run = open({ speed: 60, code: '15', number: 5, level: 2 });
  run.at(10000);
  const landed = run.last().gen;
  expect(landed).toBeLessThan(600);
  run.at(11000);
  expect(run.last().gen - landed).toBeLessThanOrEqual(60);
});

test('a speed change mid-play keeps the generation count continuous', () => {
  const run = open({ speed: 10 });
  run.at(1000);
  run.live.current = { ...run.live.current, speed: 60 };
  run.at(1016);
  expect(run.last().gen).toBeLessThanOrEqual(11);
  for (let t = 1116; t <= 2016; t += 100) run.at(t);
  expect(run.last().gen).toBe(70);
});

test('painting a blinker on a cleared board and stepping turns it, and the fate reads loop 2', () => {
  const run = open({ speed: 10 });
  run.scene.clear();
  expect(run.last()).toMatchObject({ gen: 0, pop: 0, fate: 'dead' });
  const cell = 480 * 0.96 / 128;
  const left = (640 - 128 * cell) / 2;
  const top = (480 - 128 * cell) / 2;
  const u = (c) => (left + (c + 0.5) * cell) / 640;
  const v = (r) => (top + (r + 0.5) * cell) / 480;
  run.scene.touch(u(63), v(64), true);
  run.scene.touch(u(64), v(64), false);
  run.scene.touch(u(65), v(64), false);
  expect(run.last()).toMatchObject({ pop: 3, fate: '' });
  run.scene.step();
  run.scene.step();
  expect(run.last()).toMatchObject({ gen: 2, pop: 3, fate: 'loop 2' });
  run.scene.touch(u(64), v(64), true);
  expect(run.last().pop).toBe(2);
});

test('a wolfram tap edits the newest row, and only within a thumb of it', () => {
  const run = open({ mode: 'wolfram', rule: 90, from: 'one', wrap: 0, speed: 60 }, { w: LINE * 2, h: 400 });
  const u = (127 + 10.5) / (LINE * 2);
  run.scene.touch(u, 8.5 / 400, true);
  expect(run.last().pop).toBe(2);
  run.scene.touch(u, 200 / 400, true);
  expect(run.last().pop).toBe(2);
  run.scene.touch(u, 38.5 / 400, true);
  expect(run.last().pop).toBe(2);
  run.scene.touch(u, 28.5 / 400, true);
  expect(run.last().pop).toBe(1);
});

test('the status names the rule: births and stays on the named mask, a sequence by its name, or the cube design a wolfram rule is and the plane design it draws', () => {
  expect(open({ born: '3', stay: '2 3' }).last().name).toBe('B3/S23 on carpet');
  expect(open({ born: '34-45', stay: '34-58', code: '15', number: 9 }).last().name).toBe('B34-45/S34-58');
  expect(open({ born: 'primes', stay: '23', code: '14' }).last().name).toBe('Bprimes/S23 on net');
  expect(open({ mode: 'wolfram', rule: 110 }).last().name).toBe('bang dim 3, code 110');
  expect(open({ mode: 'wolfram', rule: 90 }).last().name).toBe('bang dim 3, code 90 draws bang dim 2, code 13');
});

test('the status hands the resolved counts, so a picker can write them back by hand', () => {
  expect(open({ born: 'primes', stay: '23' }).last().counts).toEqual([[2, 3, 5, 7], [2, 3]]);
  expect(open({ mode: 'wolfram' }).last().counts).toBeNull();
});

test('wolfram rows are the unit history of the seed row, drawn by scrolling once the sheet is full', () => {
  const run = open({ mode: 'wolfram', rule: 90, from: 'one', wrap: 0, speed: 60 }, { w: LINE * 2, h: 17 });
  const seed = new Uint8Array(LINE);
  seed[LINE >> 1] = 1;
  run.at(250);
  expect([run.last().gen, run.log.some(([verb]) => verb === 'drawImage')]).toEqual([15, false]);
  run.at(500);
  expect(run.last().gen).toBe(30);
  const sheet = life.history(seed, 90, 30, false);
  const drawn = run.log.filter(([verb]) => verb === 'rect').length;
  const lit = sheet.data.subarray(16 * LINE, 31 * LINE).reduce((a, b) => a + b, 0);
  expect([run.log.some(([verb]) => verb === 'drawImage'), drawn]).toEqual([true, lit]);
});

test('under reduced motion a wolfram still fills the sheet at once and a life still runs a burst', () => {
  const run = open({ mode: 'wolfram', rule: 30, from: 'one', wrap: 0 }, { still: true, w: LINE, h: 20 });
  expect(run.last().gen).toBe(18);
  run.at(0);
  expect(run.last().gen).toBe(18);
  expect(open({ from: 'pentomino' }, { still: true }).last().gen).toBe(24);
});

test('a settled board reseeds from the seed after the hold, and never once a hand has painted', () => {
  const run = open({ from: 'blinker', speed: 10 });
  run.at(300);
  expect(run.last()).toMatchObject({ gen: 3, pop: 3, fate: 'loop 2' });
  run.at(300 + HOLD - 50);
  expect(run.last().gen).toBeGreaterThan(3);
  run.at(400 + HOLD);
  expect(run.last()).toMatchObject({ gen: 0, pop: 3, fate: '' });
  expect(run.log.some(([verb]) => verb === 'rect')).toBe(true);
  const held = open({ from: 'blinker', speed: 10 });
  held.at(300);
  held.scene.clear();
  held.at(400 + 2 * HOLD);
  expect(held.last()).toMatchObject({ pop: 0, fate: 'dead' });
  const soup = open({ speed: 60, density: 0.02 });
  soup.at(300);
  soup.at(400 + HOLD);
  expect(soup.last().gen).toBeLessThan(30);
});

test('the paints are read once at make and again on theme, never on a draw', () => {
  let looks = 0;
  const live = { current: { ...defaults(SPEC), seed: 7, speed: 60 } };
  const view = { rand: rng(7), look: () => (looks++, { paper: '#000000', accent: '#008cff' }), still: false, w: 640, h: 480, dpr: 1, t: 0 };
  const scene = make({ getContext: () => pen([]) }, view, { ...live.current, life, math, live });
  const made = looks;
  for (let t = 100; t <= 1000; t += 100) {
    view.t = t;
    scene.draw();
  }
  expect([made, looks]).toEqual([1, 1]);
  scene.theme();
  expect(looks).toBe(2);
});
