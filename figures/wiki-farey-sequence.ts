import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const ORDER = 8;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const step = frame.h / ORDER;
  const radius = step * 0.06;
  let total = 0;
  let fresh = 0;
  for (let q = 1; q <= ORDER; q++) {
    const y = frame.y + (q - 0.5) * step;
    pen.segment([frame.x, y], [frame.x + frame.w, y], 1.5, ink.line);
    for (let b = 1; b <= q; b++) {
      for (let a = 0; a <= b; a++) {
        if (num.factor.gcd(a, b) !== "1") continue;
        const x = frame.x + (frame.w * a) / b;
        const isNew = b === q;
        pen.disc(x, y, radius, isNew ? ink.yellow : ink.blue);
        total++;
        if (isNew) fresh++;
      }
    }
  }
  if (total !== 83) throw new Error(`wiki-farey-sequence: ${total} dots, want 83`);
  if (fresh !== 23) throw new Error(`wiki-farey-sequence: ${fresh} new dots, want 23`);
}
