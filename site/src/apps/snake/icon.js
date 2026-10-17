import { frame, Grid } from "#view";
import { plate } from "../../lib/plate.js";

export const units = {};

const SIDE = 5;
const CUT = 2;
const SNAKE = [[2, 0], [3, 0], [4, 0], [4, 1], [4, 2], [3, 2], [2, 2], [1, 2], [0, 2], [0, 3], [0, 4], [1, 4], [2, 4], [3, 4]];
const FOOD = [0, 0];
const HEAD = ["02220", "24242", "22222", "22222", "22222"];
const TONGUE = ["00000", "00000", "00000", "03030", "00300"];
const APPLE = ["00420", "03330", "32333", "33333", "03330"];

const STEPS = [[-1, 0], [0, 1], [1, 0], [0, -1]];

const toward = (a, b) => STEPS.findIndex(([dr, dc]) => a[0] + dr === b[0] && a[1] + dc === b[1]);

const turn = (art, k) => (k ? turn(art.map((_, r) => art.map((line) => line[r]).reverse().join("")), k - 1) : art);

function corners(i) {
  const near = [SNAKE[i - 1], SNAKE[i + 1]].filter(Boolean).map((one) => toward(SNAKE[i], one));
  if (near.some((dir) => dir < 0)) throw new Error(`snake: segment ${i} does not touch its neighbours`);
  const open = (dir) => !near.includes(dir);
  return [open(0) && open(3), open(0) && open(1), open(2) && open(1), open(2) && open(3)];
}

function body(round) {
  const end = SIDE - 1;
  return Array.from({ length: SIDE }, (_, row) => Array.from({ length: SIDE }, (_, col) => {
    const far = [row + col, row + end - col, 2 * end - row - col, end - row + col];
    return far.some((d, k) => round[k] && d < CUT) ? "0" : "1";
  }).join(""));
}

function check() {
  for (const art of [HEAD, TONGUE, APPLE]) if (art.length !== SIDE || art.some((line) => line.length !== SIDE)) throw new Error(`snake: a pixel sprite is not ${SIDE} by ${SIDE}`);
  const taken = new Set(SNAKE.map(([r, c]) => `${r},${c}`));
  if (taken.size !== SNAKE.length || taken.has(FOOD.join(","))) throw new Error("snake: the snake crosses itself or the food");
  SNAKE.forEach((a, i) => SNAKE.forEach((b, j) => {
    if (j > i + 1 && toward(a, b) >= 0) throw new Error(`snake: segments ${i} and ${j} touch across the coil`);
  }));
  if (SNAKE.some((at) => toward(at, FOOD) >= 0)) throw new Error("snake: the apple touches the body");
  const face = (toward(SNAKE[0], SNAKE[1]) + 2) % 4;
  const [dr, dc] = STEPS[face];
  const lick = [SNAKE[0][0] + dr, SNAKE[0][1] + dc];
  if (taken.has(lick.join(",")) || FOOD[0] !== lick[0] + dr || FOOD[1] !== lick[1] + dc) throw new Error("snake: the head does not face the apple across one free cell");
  return { face, lick };
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "green", 0.16);
  const { face, lick } = check();
  const tone = { 1: paint.white, 2: paint.light, 3: paint.deep, 4: paint.ink };
  const all = [...SNAKE, FOOD, lick];
  const cols = SIDE * (Math.max(...all.map(([, c]) => c)) + 1);
  const rows = SIDE * (Math.max(...all.map(([r]) => r)) + 1);
  const unit = Math.floor(Math.min(box.w / cols, box.h / rows));
  const [cx, cy] = box.center();
  const cells = new Grid(frame(Math.round(cx - (unit * cols) / 2), Math.round(cy - (unit * rows) / 2), unit * cols, unit * rows), cols, rows, 0);
  const put = (at, art) => art.forEach((line, row) => [...line].forEach((c, col) => c !== "0" && cells.fill(pen, at[1] * SIDE + col, at[0] * SIDE + row, tone[c])));
  put(FOOD, APPLE);
  put(lick, turn(TONGUE, face));
  SNAKE.forEach((at, i) => put(at, i ? body(corners(i)) : turn(HEAD, face)));
}
