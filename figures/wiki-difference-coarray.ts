import * as num from "mrlyjs/num";
import type { Color, Ink, Pen } from "mrlyjs/view";

export const units = { num };

const RULER = [0, 1, 4, 6];
const SPAN = 6;
const PAIRS = 16;
const LAGS = 13;
const OFF = 1.4;
const GAP = 1.5;
const SHRINK = 0.84;
const BASE = 0.2;
const RULE = 2;

const lit = (p: number) => RULER.includes(p);

function weights() {
  const marks = Array.from({ length: LAGS }, (_, p) => (lit(p) ? 1 : 0));
  const mirror = Array.from({ length: LAGS }, (_, m) => (m <= SPAN ? marks[SPAN - m] : 0));
  return num.blend.cauchy(mirror, marks).map(Number);
}

function diamond(pen: Pen, [x, y]: [number, number], h: number, color: Color) {
  const r = h * SHRINK;
  pen.polygon([[x, y - r], [x + r, y], [x, y + r], [x - r, y]], color);
}

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const counts = weights();
  if (counts.length !== LAGS) throw new Error(`wiki-difference-coarray: ${counts.length} lags, want ${LAGS}`);
  if (!counts.every((w) => w > 0)) throw new Error("wiki-difference-coarray: a lag has no pair");
  if (counts[SPAN] !== RULER.length) throw new Error(`wiki-difference-coarray: lag zero holds ${counts[SPAN]}, want ${RULER.length}`);
  const singles = counts.filter((w) => w === 1).length;
  if (singles !== LAGS - 1) throw new Error(`wiki-difference-coarray: ${singles} single lags, want ${LAGS - 1}`);
  const total = counts.reduce((sum, w) => sum + w, 0);
  if (total !== PAIRS) throw new Error(`wiki-difference-coarray: ${total} pairs, want ${PAIRS}`);
  const tall = Math.max(...counts);
  const top = -OFF - SHRINK;
  const floor = 2 * SPAN + 1 + GAP + tall;
  const base = floor + BASE;
  const wide = 2 * (SPAN + OFF + SHRINK);
  const side = Math.min(frame.w, frame.h) - RULE / 2;
  const h = side / Math.max(wide, base - top);
  const bottom = base + RULE / 2 / h;
  const [cx, cy] = frame.center();
  const y0 = cy - (h * (top + bottom)) / 2;
  const at = (u: number, v: number): [number, number] => [cx + u * h, y0 + v * h];
  let pairs = 0;
  for (let i = 0; i <= SPAN; i++) {
    for (let j = 0; j <= SPAN; j++) {
      const on = lit(i) && lit(j);
      if (on) pairs++;
      diamond(pen, at(j - i, i + j), h, on ? ink.blue : ink.line);
    }
  }
  if (pairs !== PAIRS) throw new Error(`wiki-difference-coarray: ${pairs} lit pairs, want ${PAIRS}`);
  for (let p = 0; p <= SPAN; p++) {
    const color = lit(p) ? ink.yellow : ink.line;
    diamond(pen, at(-OFF - p, p - OFF), h, color);
    diamond(pen, at(p + OFF, p - OFF), h, color);
  }
  const block = h * SHRINK;
  counts.forEach((w, k) => {
    const [x] = at(k - SPAN, 0);
    for (let s = 0; s < w; s++) {
      const [, y] = at(0, floor - s - 0.5);
      pen.rect(x - block / 2, y - block / 2, block, block, ink.blue);
    }
  });
  const [left, y] = at(-SPAN - 0.5, base);
  const [right] = at(SPAN + 0.5, base);
  pen.segment([left, y], [right, y], RULE, ink.line);
}
