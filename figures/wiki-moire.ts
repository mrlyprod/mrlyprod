import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const SMALL = 5;
const LARGE = 7;

const marks = (n: number) => Array.from({ length: n + 1 }, (_, k) => k / n);

export default function draw(pen: Pen, ink: Ink) {
  const common = Number(num.factor.gcd(SMALL, LARGE));
  const shared = marks(common);
  const thin = marks(SMALL);
  const thick = marks(LARGE);
  const crossings: [number, number][] = [];
  for (const x of thin) {
    for (const y of thick) {
      crossings.push([x, y]);
      if (!shared.includes(y) || !shared.includes(x)) crossings.push([y, x]);
    }
  }
  if (common !== 1) throw new Error(`wiki-moire: gcd ${common}, want 1`);
  if (shared.length !== 2) throw new Error(`wiki-moire: ${shared.length} shared marks, want 2`);
  if (crossings.length !== 92) throw new Error(`wiki-moire: ${crossings.length} crossings, want 92`);

  const frame = pen.frame(0.1);
  for (const [scale, tone] of [
    [thin, ink.blue],
    [thick, ink.orange],
  ] as const) {
    for (const u of scale) {
      const [x, top] = frame.at(u, 0);
      const [left, y] = frame.at(0, u);
      pen.segment([x, top], [x, top + frame.h], 5, tone);
      pen.segment([left, y], [left + frame.w, y], 5, tone);
    }
  }
  for (const [u, v] of crossings) {
    const [x, y] = frame.at(u, v);
    pen.disc(x, y, frame.w * 0.01, ink.fade(ink.dim, 0.9));
  }
  for (const u of shared) {
    for (const v of shared) {
      const [x, y] = frame.at(u, v);
      pen.disc(x, y, frame.w * 0.035, ink.yellow);
    }
  }
}
