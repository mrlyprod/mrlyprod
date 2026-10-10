import * as num from "mrlyjs/num";
import { Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const SIDE = 24;

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const grid = new Grid(area, SIDE, SIDE, 0.1);
  const ramp = ink.Ramp.tone(ink.dim, ink.blue);
  let lit = 0;
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const a = col + 1;
      const b = SIDE - row;
      const share = Number(num.factor.gcd(a, b));
      if (share === 1) lit++;
      const tone = share === 1 ? ink.yellow : ramp.at((share - 2) / (SIDE - 2));
      grid.fill(pen, col, row, tone);
    }
  }
  const pairs = num.lattice.coprime_pairs(SIDE);
  if (BigInt(lit) !== pairs) throw new Error(`wiki-greatest-common-divisor: ${lit} coprime cells, want ${pairs}`);
  if (lit !== 359) throw new Error(`wiki-greatest-common-divisor: ${lit} coprime cells, want 359`);
}
