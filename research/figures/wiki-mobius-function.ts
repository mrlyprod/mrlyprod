import * as num from "mrlyjs/num";
import { Grid, type Ink, type Pen } from "mrlyjs/view";

export const units = { num };

const TOP = 100;
const SIDE = 10;

export default function draw(pen: Pen, ink: Ink) {
  const mu = num.factor.mobius_sieve(TOP);
  const grid = new Grid(pen.frame(0.08), SIDE, SIDE, 0.1);
  let plus = 0;
  let minus = 0;
  let flat = 0;
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const number = row * SIDE + col + 1;
      if (mu[number] !== num.factor.mobius(number)) throw new Error(`wiki-mobius-function: the sieve gives ${mu[number]} at ${number}, want ${num.factor.mobius(number)}`);
      let tone = ink.dim;
      if (mu[number] === 1) {
        plus++;
        tone = ink.yellow;
      } else if (mu[number] === -1) {
        minus++;
        tone = ink.blue;
      } else {
        flat++;
      }
      grid.fill(pen, col, row, tone);
    }
  }
  if (plus !== 31 || minus !== 30 || flat !== 39) throw new Error(`wiki-mobius-function: tallies ${plus}, ${minus}, ${flat}, want 31, 30, 39`);
}
