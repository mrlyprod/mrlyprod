import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const SIDE = 101;
const REACH = 50;
const PRIMES = 1252;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.06);
  const unit = frame.w / SIDE;
  const top = SIDE * SIDE;
  const prime = num.prime.flags(top);
  let lit = 0;
  for (let n = 1; n <= top; n++) {
    if (!prime[n]) continue;
    const [x, y] = num.spiral.Lattice.xy("Square", n);
    const px = frame.x + (Number(x) + REACH) * unit;
    const py = frame.y + (REACH - Number(y)) * unit;
    pen.rect(px, py, unit * 0.86, unit * 0.86, ink.blue);
    lit++;
  }
  if (lit !== PRIMES) throw new Error(`wiki-ulam-spiral: ${lit} primes lit, want ${PRIMES}`);
}
