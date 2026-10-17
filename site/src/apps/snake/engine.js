export const UP = 0;
export const RIGHT = 1;
export const DOWN = 2;
export const LEFT = 3;

const DX = [0, 1, 0, -1];
const DY = [-1, 0, 1, 0];
export const DIRS = [UP, RIGHT, DOWN, LEFT];
const LIMIT = 3;

/* CELLS */

export const flip = (dir) => (dir + 2) & 3;

export function ahead(size, wrap, cell, dir) {
  let x = (cell % size) + DX[dir];
  let y = Math.floor(cell / size) + DY[dir];
  if (wrap) {
    x = (x + size) % size;
    y = (y + size) % size;
  } else if (x < 0 || x >= size || y < 0 || y >= size) {
    return -1;
  }
  return y * size + x;
}

export function bearing(size, from, to) {
  return DIRS.find((dir) => ahead(size, 1, from, dir) === to);
}

function drop(size, taken, rand) {
  const used = new Uint8Array(size * size);
  for (const cell of taken) used[cell] = 1;
  const free = [];
  for (let cell = 0; cell < used.length; cell++) if (!used[cell]) free.push(cell);
  return free.length ? free[Math.floor(rand() * free.length)] : -1;
}

/* RULES */

export function start({ size, apples, wrap }, rand) {
  const dir = Math.floor(rand() * 4);
  const head = (size >> 1) * size + (size >> 1);
  const snake = [head, ahead(size, wrap, head, flip(dir))];
  const foods = [];
  for (let i = 0; i < apples; i++) {
    const spot = drop(size, [...snake, ...foods], rand);
    if (spot < 0) break;
    foods.push(spot);
  }
  return { size, wrap, snake, dir, queue: [], foods, score: 0, hunger: 0, over: null };
}

export function turn(state, dir) {
  if (state.over || !DIRS.includes(dir) || state.queue.length >= LIMIT) return state;
  const last = state.queue.length ? state.queue[state.queue.length - 1] : state.dir;
  if (dir === last || dir === flip(last)) return state;
  return { ...state, queue: [...state.queue, dir] };
}

export function step(state, rand) {
  if (state.over) return state;
  const { size, wrap } = state;
  const queue = state.queue.slice();
  const dir = queue.length ? queue.shift() : state.dir;
  const head = ahead(size, wrap, state.snake[0], dir);
  if (head < 0) return { ...state, dir, queue, over: 'wall' };
  const eat = state.foods.indexOf(head);
  const body = eat >= 0 ? state.snake : state.snake.slice(0, -1);
  if (body.includes(head)) return { ...state, dir, queue, over: 'bite' };
  const snake = [head, ...body];
  if (eat < 0) return { ...state, snake, dir, queue, hunger: state.hunger + 1 };
  const foods = state.foods.slice();
  const spot = drop(size, [...snake, ...foods.filter((_, i) => i !== eat)], rand);
  if (spot < 0) foods.splice(eat, 1);
  else foods[eat] = spot;
  const over = snake.length === size * size ? 'won' : null;
  return { ...state, snake, dir, queue, foods, score: state.score + 1, hunger: 0, over };
}

export function starve(state) {
  return state.over ? state : { ...state, over: 'starved' };
}

/* ROUNDS */

const SQUARE = [0, 0, 0, 0];
const ENDS = [
  [0, 0, 1, 1],
  [1, 0, 0, 1],
  [1, 1, 0, 0],
  [0, 1, 1, 0],
];
const CORNERS = {
  [(1 << UP) | (1 << RIGHT)]: [0, 0, 0, 1],
  [(1 << UP) | (1 << LEFT)]: [0, 0, 1, 0],
  [(1 << DOWN) | (1 << RIGHT)]: [1, 0, 0, 0],
  [(1 << DOWN) | (1 << LEFT)]: [0, 1, 0, 0],
};

export function rounds(state) {
  const { size, snake } = state;
  const last = snake.length - 1;
  return snake.map((cell, i) => {
    if (last < 1) return SQUARE;
    if (i === 0) return ENDS[bearing(size, cell, snake[1])] ?? SQUARE;
    if (i === last) return ENDS[bearing(size, cell, snake[i - 1])] ?? SQUARE;
    const mask = (1 << bearing(size, cell, snake[i - 1])) | (1 << bearing(size, cell, snake[i + 1]));
    return CORNERS[mask] ?? SQUARE;
  });
}
