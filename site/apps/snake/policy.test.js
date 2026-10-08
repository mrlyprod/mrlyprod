import { expect, test } from 'bun:test';
import { rng } from '../../lib/scene.js';
import { DOWN, LEFT, flip, start, step, turn } from './engine.js';
import { STARVED, pick } from './policy.js';

const drive = (state, mode, rand) => step(turn(state, pick(state, mode, rand)), rand);

const BAIT = {
  size: 8,
  wrap: 1,
  snake: [63, 55, 47, 39, 31, 23, 15, 14, 6, 5, 13, 12, 4, 3, 11, 10, 2, 1, 9, 17, 18, 19, 20, 21, 22, 30, 29, 28, 27, 26, 25, 33, 34, 35, 43, 42, 41, 40, 48, 56, 0, 8],
  dir: DOWN,
  queue: [],
  foods: [7],
  score: 40,
  hunger: 0,
  over: null,
};

test('smart reaches the food on an empty board within size*2 steps', () => {
  for (const wrap of [1, 0]) {
    for (let seed = 1; seed <= 10; seed++) {
      const rand = rng(seed);
      let state = start({ size: 16, apples: 1, wrap }, rand);
      let steps = 0;
      while (!state.score && !state.over && steps <= 32) {
        state = drive(state, 'smart', rand);
        steps++;
      }
      expect([wrap, seed, state.score, steps <= 32]).toEqual([wrap, seed, 1, true]);
    }
  }
});

test('smart never dies in 200 steps on a 10x10 board', () => {
  for (const wrap of [1, 0]) {
    for (let seed = 1; seed <= 10; seed++) {
      const rand = rng(seed);
      let state = start({ size: 10, apples: 1, wrap }, rand);
      for (let i = 0; i < 200 && !state.over; i++) state = drive(state, 'smart', rand);
      expect([wrap, seed, state.over]).toEqual([wrap, seed, null]);
    }
  }
});

test('silly never reverses in 200 steps', () => {
  const rand = rng(5);
  let state = start({ size: 10, apples: 1, wrap: 1 }, rand);
  for (let i = 0; i < 200; i++) {
    if (state.over) state = start({ size: 10, apples: 1, wrap: 1 }, rand);
    const dir = pick(state, 'silly', rand);
    expect(dir).not.toBe(flip(state.dir));
    state = step(turn(state, dir), rand);
  }
});

test('smart never chases its tail through a food it cannot safely eat', () => {
  const picks = new Set(Array.from({ length: 20 }, (_, i) => pick(BAIT, 'smart', rng(i + 1))));
  expect(picks.has(DOWN)).toBe(false);
});

test('a hungry smart takes the food it refused, and starves with no way to any', () => {
  const rand = rng(1);
  expect(pick({ ...BAIT, hunger: 64 }, 'smart', rand)).toBe(DOWN);
  const boxed = { size: 8, wrap: 0, snake: [0, 1, 9, 8, 16, 24], dir: LEFT, queue: [], foods: [63], score: 4, hunger: 64, over: null };
  expect(pick(boxed, 'smart', rand)).toBe(STARVED);
});
