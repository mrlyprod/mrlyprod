import * as num from "@mrlyprod/mrly/num";
import type { Pen, Point } from "#view";
import { Grid } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = { num };

/* TYPES */

type Sheet = { xs: Int32Array; ys: Int32Array; marks: Int8Array; line: number[] };

const SIDE = 15;
const DOT = 0.38;
const LINE = 0.52;
const TRAIL = 0.12;
const GHOST = 70;
const WANT = { top: 225, primes: 48, line: 10, sum: -2 };

let sheet: Sheet | null = null;

function facts(): Sheet {
  if (sheet) return sheet;
  const top = num.spiral.Lattice.count("Square", SIDE);
  const marks = num.spiral.marks("Prime", top);
  const xs = new Int32Array(top + 1);
  const ys = new Int32Array(top + 1);
  const sums = new Map<number, number[]>();
  let lit = 0;
  for (let n = 1; n <= top; n++) {
    const [x, y] = num.spiral.Lattice.xy("Square", n);
    xs[n] = Number(x);
    ys[n] = Number(y);
    if (n > 1 && Math.abs(xs[n] - xs[n - 1]) + Math.abs(ys[n] - ys[n - 1]) !== 1)
      throw new Error(`ulam: the winding jumps at ${n}`);
    if (marks[n] !== 1) continue;
    lit++;
    const sum = xs[n] + ys[n];
    sums.set(sum, [...(sums.get(sum) ?? []), n]);
  }
  if (top !== WANT.top || lit !== WANT.primes || lit !== num.prime.prime_count(top))
    throw new Error(
      `ulam: a sheet of ${SIDE} holds ${top} numbers and ${lit} primes, want ${WANT.top} and ${WANT.primes}`,
    );
  const [sum, line] = [...sums].reduce((best, next) => (next[1].length > best[1].length ? next : best));
  if (sum !== WANT.sum || line.length !== WANT.line)
    throw new Error(
      `ulam: the richest diagonal is x + y = ${sum} with ${line.length} primes, want ${WANT.sum} and ${WANT.line}`,
    );
  sheet = { xs, ys, marks, line };
  return sheet;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "yellow", 0.16);
  const { xs, ys, marks, line } = facts();
  const cells = new Grid(box, SIDE, SIDE, 0);
  const half = (SIDE - 1) / 2;
  const unit = box.w / SIDE;
  const middle = (n: number): Point => {
    const [x, y, w, h] = cells.cell(xs[n] + half, half - ys[n]);
    return [x + w / 2, y + h / 2];
  };
  const trail: Point[] = [];
  for (let n = 1; n <= WANT.top; n++) trail.push(middle(n));
  pen.polyline(trail, unit * TRAIL, [255, 255, 255, GHOST]);
  const ends = line.map(middle).sort((a, b) => a[0] - b[0]);
  pen.segment(ends[0], ends.at(-1)!, unit * LINE, paint.light);
  for (let n = 2; n <= WANT.top; n++) {
    if (marks[n] !== 1) continue;
    const [x, y] = middle(n);
    pen.disc(x, y, unit * DOT, paint.white);
  }
}
