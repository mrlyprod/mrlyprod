import * as math from "mrlyjs/math";
import * as num from "mrlyjs/num";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { math, num };

const CODE = 7;
const BASE = 2;
const NUMBER = 2;
const TOP = 100;
const GROWTH = "Every";
const MARGIN = 0.08;
const FAINT = 0.45;
const FILL = 0.08;
const EDGE = 0.35;
const TRAIL = 0.5;
const HAIR = 2;
const LINE = 3;
const DOT = 7;
const WANT = { tiles: 100, primes: 25, levels: [1, 2, 4, 8, 16, 32, 37], box: 640, lit: 3 ** 6 };

type Shell = ReturnType<typeof num.spiral.snail>;
type Run = [number, number, number];

let memo: { shell: Shell; lines: Run[][] } | null = null;

function runs(cell: { shape: number[]; types: ArrayLike<number> }): Run[] {
  const [rows, cols] = cell.shape;
  const out: Run[] = [];
  for (let row = 0; row < rows; row++) {
    let start = -1;
    for (let col = 0; col <= cols; col++) {
      const on = col < cols && cell.types[row * cols + col];
      if (on && start < 0) start = col;
      if (!on && start >= 0) {
        out.push([row, start, col - start]);
        start = -1;
      }
    }
  }
  return out;
}

function build() {
  if (memo) return memo;
  const shell = num.spiral.snail(NUMBER, TOP, GROWTH);
  if (shell.tiles.length !== WANT.tiles || shell.primes !== WANT.primes || shell.levels.join() !== WANT.levels.join()) throw new Error(`app-snail: ${shell.tiles.length} tiles, ${shell.primes} primes, levels ${shell.levels.join()}, want ${WANT.tiles}, ${WANT.primes}, ${WANT.levels.join()}`);
  if (shell.high[0] - shell.low[0] !== WANT.box || shell.high[1] - shell.low[1] !== WANT.box) throw new Error(`app-snail: the box is ${shell.high[0] - shell.low[0]} by ${shell.high[1] - shell.low[1]}, want ${WANT.box} square`);
  const lines = shell.levels.map((_, level) => (level ? runs(math.two.create(CODE, NUMBER, level, 0, BASE)) : []));
  const lit = lines[6].reduce((sum, [, , len]) => sum + len, 0);
  if (lit !== WANT.lit) throw new Error(`app-snail: the carpet of 2 at level 6 lights ${lit} cells, want ${WANT.lit}`);
  memo = { shell, lines };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { shell, lines } = build();
  const box = pen.frame(MARGIN);
  const k = box.w / WANT.box;
  const X = (x: number) => box.x + (x - shell.low[0]) * k;
  const Y = (y: number) => box.y + (shell.high[1] - y) * k;
  const faint = ink.mix(ink.ground, ink.blue, FAINT);
  const path: Point[] = [];
  for (const tile of shell.tiles) {
    const { x, y, side, level, prime } = tile;
    const tone = prime ? ink.blue : faint;
    path.push([X(x + side / 2), Y(y + side / 2)]);
    if (!level) {
      pen.rect(X(x), Y(y + side), side * k, side * k, tone);
      continue;
    }
    pen.rect(X(x), Y(y + side), side * k, side * k, ink.mix(ink.ground, tone, FILL));
    for (const [row, col, len] of lines[level]) pen.rect(X(x + col), Y(y + side - row), len * k, k, tone);
    const edge = ink.mix(ink.ground, tone, EDGE);
    pen.polyline([[X(x), Y(y + side)], [X(x + side), Y(y + side)], [X(x + side), Y(y)], [X(x), Y(y)], [X(x), Y(y + side)]], HAIR, edge);
  }
  pen.polyline(path, LINE, ink.mix(ink.ground, ink.blue, TRAIL));
  pen.disc(path[path.length - 1][0], path[path.length - 1][1], DOT, ink.fg);
}
