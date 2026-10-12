import * as math from "mrlyjs/math";
import * as num from "mrlyjs/num";
import { Grid, type Ink, type Pen } from "mrlyjs/view";

export const units = { math, num };

const SIDE = 32;

export default function draw(pen: Pen, ink: Ink) {
  const gasket = math.two.from_corners([[0, 0], [1, 0], [0, 1]], 2, 5, 0, 2);
  if (gasket.shape.join() !== `${SIDE},${SIDE}`) throw new Error(`paper-coprime-density-above-dimension-one: gasket ${gasket.shape}, want 32,32`);
  const types = gasket.types;
  const grid = new Grid(pen.frame(0.08), SIDE, SIDE, 0.1);
  let filled = 0;
  let visible = 0;
  for (let y = 0; y < SIDE; y++) {
    for (let x = 0; x < SIDE; x++) {
      if (types[y * SIDE + x] === 0) continue;
      filled++;
      const coprime = num.factor.gcd(x, y) === "1";
      if (coprime) visible++;
      grid.fill(pen, x, SIDE - 1 - y, coprime ? ink.yellow : ink.dim);
    }
  }
  if (filled !== 243 || visible !== 122) throw new Error(`paper-coprime-density-above-dimension-one: ${filled} filled and ${visible} coprime, want 243 and 122`);
}
