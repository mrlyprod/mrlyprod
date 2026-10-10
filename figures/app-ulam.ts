import * as num from "mrlyjs/num";
import { Grid } from "mrlyjs/view";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { num };

const SIDE = 31;
const MARGIN = 0.08;
const GAP = 0.12;
const THICK = 3;
const PATH = 0.5;
const WANT = { top: 961, primes: 162 };

type Sheet = { xs: Int32Array; ys: Int32Array; marks: Int8Array };

let sheet: Sheet | null = null;

function facts(): Sheet {
  if (sheet) return sheet;
  const top = num.spiral.Lattice.count("Square", SIDE);
  const marks = num.spiral.marks("Prime", top);
  const xs = new Int32Array(top + 1);
  const ys = new Int32Array(top + 1);
  let lit = 0;
  for (let n = 1; n <= top; n++) {
    const [x, y] = num.spiral.Lattice.xy("Square", n);
    xs[n] = Number(x);
    ys[n] = Number(y);
    if (n > 1 && Math.abs(xs[n] - xs[n - 1]) + Math.abs(ys[n] - ys[n - 1]) !== 1) throw new Error(`app-ulam: the winding jumps at ${n}`);
    if (marks[n] === 1) lit++;
  }
  if (top !== WANT.top || lit !== WANT.primes || lit !== num.prime.prime_count(top)) throw new Error(`app-ulam: a sheet of ${SIDE} holds ${top} numbers and ${lit} primes, want ${WANT.top} and ${WANT.primes}`);
  sheet = { xs, ys, marks };
  return sheet;
}

export default function draw(pen: Pen, ink: Ink) {
  const { xs, ys, marks } = facts();
  const cells = new Grid(pen.frame(MARGIN), SIDE, SIDE, GAP);
  const half = (SIDE - 1) / 2;
  const middle = (n: number): Point => {
    const [x, y, w, h] = cells.cell(xs[n] + half, half - ys[n]);
    return [x + w / 2, y + h / 2];
  };
  const trail: Point[] = [];
  for (let n = 1; n <= WANT.top; n++) trail.push(middle(n));
  pen.polyline(trail, THICK, ink.mix(ink.ground, ink.dim, PATH));
  for (let n = 1; n <= WANT.top; n++) if (marks[n] === 1) cells.fill(pen, xs[n] + half, half - ys[n], ink.blue);
}
