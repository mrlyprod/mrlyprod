import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const ORDER = 8;
const THICK = 2.6;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.06);
  const ramp = ink.Ramp.tone(ink.blue, ink.yellow);
  const base = frame.y + frame.h;
  let circles = 0;
  for (let b = 1; b <= ORDER; b++) {
    for (let a = 0; a <= b; a++) {
      if (num.factor.gcd(a, b) !== "1") continue;
      const r = frame.w / (2 * b * b);
      const x = frame.x + (frame.w * a) / b;
      const tone = ramp.at((b - 1) / (ORDER - 1));
      pen.ring(x, base - r, r, THICK, tone);
      pen.disc(x, base, THICK * 1.3, tone);
      circles++;
    }
  }
  pen.segment([frame.x, base], [frame.x + frame.w, base], 1.6, ink.line);
  if (circles !== 23) throw new Error(`wiki-ford-circles: ${circles} circles, want 23`);
}
