import * as num from "mrlyjs/num";
import { plot, type Ink, type Pen, type Point } from "mrlyjs/view";

const X = 36;
const ROOT = 6;
const WINDOW = 38;
const SAMPLES = 800;
const TOTAL = 140;
const KNOWN: [number, number][] = [
  [36, 9],
  [31, 2],
];

export const units = { num };

let memo: Point[] | undefined;

function lattice() {
  if (memo) return memo;
  const columns: number[][] = Array.from({ length: X }, () => []);
  const counts: number[] = [];
  for (let n = 1; n <= X; n++) {
    const divisors = Array.from(num.factor.divisors(n), Number);
    counts.push(divisors.length);
    for (const a of divisors) columns[a - 1].push(n / a);
  }
  const total = counts.reduce((sum, count) => sum + count, 0);
  const folded = 2 * columns.slice(0, ROOT).reduce((sum, column) => sum + column.length, 0) - ROOT * ROOT;
  if (ROOT * ROOT !== X) throw new Error(`wiki-divisor-function: root ${ROOT} squared is not ${X}`);
  if (total !== TOTAL) throw new Error(`wiki-divisor-function: ${total} divisors in all, want ${TOTAL}`);
  if (folded !== total) throw new Error(`wiki-divisor-function: folded count ${folded}, want ${total}`);
  for (const [n, want] of KNOWN) {
    if (counts[n - 1] !== want) throw new Error(`wiki-divisor-function: ${n} has ${counts[n - 1]} divisors, want ${want}`);
  }
  memo = columns.flatMap((column, i) => column.map((b): Point => [i + 1, b]));
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const unit = frame.w / WINDOW;
  const at = (a: number, b: number): Point => [frame.x + a * unit, frame.y + frame.h - b * unit];

  const inner: Point[] = [];
  const arms: Point[] = [];
  for (const [a, b] of lattice()) (a <= ROOT && b <= ROOT ? inner : arms).push(at(a, b));
  if (inner.length !== ROOT * ROOT) throw new Error(`wiki-divisor-function: ${inner.length} inner points, want ${ROOT * ROOT}`);
  if (arms.length !== TOTAL - ROOT * ROOT) throw new Error(`wiki-divisor-function: ${arms.length} arm points, want ${TOTAL - ROOT * ROOT}`);

  const curve: Point[] = Array.from({ length: SAMPLES + 1 }, (_, i) => {
    const a = X ** (i / SAMPLES);
    return at(a, X / a);
  });
  const hair = Math.max(frame.w / 512, 1);
  pen.segment(at(0, 0), at(WINDOW, 0), hair, ink.line);
  pen.segment(at(0, 0), at(0, WINDOW), hair, ink.line);
  pen.polyline(curve, 3, ink.dim);
  const r = unit * 0.36;
  plot.dots(pen, arms, r, ink.blue);
  plot.dots(pen, inner, r, ink.yellow);
}
