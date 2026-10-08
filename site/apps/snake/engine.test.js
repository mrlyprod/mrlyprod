import { expect, test } from 'bun:test';
import { rng } from '../../lib/scene.js';
import { DOWN, LEFT, RIGHT, UP, rounds, start, step, turn } from './engine.js';

const SIZE = 8;

const freeze = (x) => {
  if (x && typeof x === 'object') {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};
const at = (x, y) => y * SIZE + x;
const board = (over) =>
  freeze({ size: SIZE, wrap: 1, snake: [at(3, 3), at(2, 3)], dir: RIGHT, queue: [], foods: [], score: 0, hunger: 0, over: null, ...over });

test('wrap carries the head to the far edge', () => {
  const next = step(board({ snake: [at(7, 3), at(6, 3)] }), rng(1));
  expect(next.snake).toEqual([at(0, 3), at(7, 3)]);
  expect(next.over).toBe(null);
});

test('a wall kills when wrap is off and leaves the snake where it was', () => {
  const state = board({ wrap: 0, snake: [at(7, 3), at(6, 3)] });
  const next = step(state, rng(1));
  expect(next.over).toBe('wall');
  expect(next.snake).toEqual(state.snake);
});

test('the head biting the body kills', () => {
  const state = board({ snake: [at(3, 3), at(2, 3), at(2, 4), at(3, 4), at(4, 4)], dir: DOWN });
  expect(step(state, rng(1)).over).toBe('bite');
});

test('the tail cell is free on a move that does not eat', () => {
  const state = board({ snake: [at(3, 3), at(3, 4), at(4, 4), at(4, 3)], dir: RIGHT });
  const next = step(state, rng(1));
  expect(next.over).toBe(null);
  expect(next.snake).toEqual([at(4, 3), at(3, 3), at(3, 4), at(4, 4)]);
});

test('eating keeps the tail, raises the score and redraws the food on a free cell', () => {
  const state = board({ foods: [at(4, 3)] });
  const next = step(state, rng(1));
  expect(next.snake).toEqual([at(4, 3), at(3, 3), at(2, 3)]);
  expect(next.score).toBe(1);
  expect(next.foods).toHaveLength(1);
  expect(next.snake).not.toContain(next.foods[0]);
});

test('the queue holds three turns and refuses a repeat, a reverse and a fourth', () => {
  let state = board();
  const queued = [];
  for (const dir of [UP, UP, DOWN, LEFT, DOWN, RIGHT]) {
    state = turn(state, dir);
    queued.push([...state.queue]);
  }
  expect(queued).toEqual([[UP], [UP], [UP], [UP, LEFT], [UP, LEFT, DOWN], [UP, LEFT, DOWN]]);
});

test('eating the last free cell is a win', () => {
  const path = [];
  for (let y = 0; y < SIZE; y++) for (let x = 0; x < SIZE; x++) path.push(at(y % 2 ? SIZE - 1 - x : x, y));
  const state = board({ snake: path.slice(0, -1).reverse(), dir: LEFT, foods: [path[SIZE * SIZE - 1]] });
  const next = step(state, rng(1));
  expect(next.over).toBe('won');
  expect(next.snake).toHaveLength(SIZE * SIZE);
  expect(next.foods).toEqual([]);
});

test('start centres a two-segment snake with its apples', () => {
  const state = start({ size: 16, apples: 10, wrap: 0 }, rng(3));
  expect(state.wrap).toBe(0);
  expect(state.foods).toHaveLength(10);
  expect(state.snake).toHaveLength(2);
  expect(state.snake[0]).toBe(8 * 16 + 8);
  expect(new Set([...state.snake, ...state.foods]).size).toBe(12);
});

test('rounds marks the head, a corner, a straight run and the tail', () => {
  const state = board({ snake: [at(3, 3), at(2, 3), at(2, 4), at(2, 5)] });
  expect(rounds(state)).toEqual([
    [0, 1, 1, 0],
    [1, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 1, 1],
  ]);
});
