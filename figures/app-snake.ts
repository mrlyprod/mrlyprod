import { frame, Grid, grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const MARK = 5;
const CELL = 40;
const SNAKE = [[0, 2], [0, 3], [1, 3], [2, 3], [2, 2], [2, 1]];
const FOOD = [0, 0];

const STEPS = [[-1, 0], [0, 1], [1, 0], [0, -1]];

const toward = (a: number[], b: number[]) => STEPS.findIndex(([dr, dc]) => a[0] + dr === b[0] && a[1] + dc === b[1]);

function corners(i: number): boolean[] {
  const near = [SNAKE[i - 1], SNAKE[i + 1]].filter(Boolean).map((one) => toward(SNAKE[i], one));
  if (near.some((dir) => dir < 0)) throw new Error(`app-snake: segment ${i} does not touch its neighbours`);
  const open = (dir: number) => !near.includes(dir);
  return [open(0) && open(3), open(0) && open(1), open(2) && open(1), open(2) && open(3)];
}

function cut(row: number, col: number, round: boolean[]): boolean {
  const end = MARK - 1;
  const at = [row === 0 && col === 0, row === 0 && col === end, row === end && col === end, row === end && col === 0];
  return at.some((corner, k) => corner && round[k]);
}

export default function draw(pen: Pen, ink: Ink) {
  const mark = grid.LOGO.map((line) => [...line].map((bit) => bit === "1"));
  if (mark.length !== MARK || mark.some((line) => line.length !== MARK)) throw new Error(`app-snake: the mark is not ${MARK} by ${MARK}`);
  const taken = new Set(SNAKE.map(([r, c]) => `${r},${c}`));
  if (taken.size !== SNAKE.length || taken.has(FOOD.join(","))) throw new Error("app-snake: the snake crosses itself or the food");
  const all = [...SNAKE, FOOD];
  const cols = MARK * (Math.max(...all.map(([, c]) => c)) + 1);
  const rows = MARK * (Math.max(...all.map(([r]) => r)) + 1);
  const cells = new Grid(frame(Math.round((pen.width - CELL * cols) / 2), Math.round((pen.height - CELL * rows) / 2), CELL * cols, CELL * rows), cols, rows, 0);
  const put = (at: number[], tone: typeof ink.fg, keep: (row: number, col: number) => boolean) => {
    for (let row = 0; row < MARK; row++) {
      for (let col = 0; col < MARK; col++) if (mark[row][col] && keep(row, col)) cells.fill(pen, at[1] * MARK + col, at[0] * MARK + row, tone);
    }
  };
  const mid = (MARK - 1) / 2;
  put(FOOD, ink.orange, (row, col) => Math.hypot(row - mid, col - mid) <= MARK / 2);
  SNAKE.forEach((at, i) => {
    const round = corners(i);
    put(at, i ? ink.blue : ink.fg, (row, col) => !cut(row, col, round));
  });
}
