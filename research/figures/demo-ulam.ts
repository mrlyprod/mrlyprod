import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const SIDE = 201;
const REACH = 100;
const UNIT = 4;
const DOT = 3;

export default function draw(pen: Pen, ink: Ink) {
  const span = SIDE * UNIT;
  const edge = (pen.width - span) / 2;
  const corner = (x: number, y: number) => [edge + (x + REACH) * UNIT, edge + (REACH - y) * UNIT];
  const line = num.spiral.diagonal("Square", SIDE, 4, -2, 41);
  if (line.top !== 40401 || line.primes !== 4236) throw new Error(`demo-ulam: top ${line.top} with ${line.primes} primes, want 40401 with 4236`);
  if (line.values.length !== 101 || line.hits !== 80 || line.streak !== 21) throw new Error(`demo-ulam: ${line.values.length} values, ${line.hits} hits, streak ${line.streak}, want 101, 80, 21`);
  for (const [cx, cy] of line.cells) {
    const [x, y] = corner(cx, cy);
    pen.rect(x - 2, y - 2, UNIT + 4, UNIT + 4, ink.orange);
  }
  const prime = num.prime.flags(line.top);
  let lit = 0;
  for (let n = 1; n <= line.top; n++) {
    if (!prime[n]) continue;
    const [px, py] = num.spiral.Lattice.xy("Square", n);
    const [x, y] = corner(Number(px), Number(py));
    pen.rect(x, y, DOT, DOT, ink.blue);
    lit++;
  }
  if (lit !== line.primes) throw new Error(`demo-ulam: ${lit} lit, want ${line.primes}`);
}
