import * as num from "mrlyjs/num";
import { Grid, type Ink, type Pen } from "mrlyjs/view";

export const units = { num };

const LIMIT = 400;
const SIDE = 20;

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const sieve = new num.prime.Sieve(LIMIT);
  sieve.finish();
  const [count, rank, types] = [sieve.count(), sieve.rank(), sieve.types()];
  sieve.free();
  if (count !== 78) throw new Error(`demo-primes: ${count} primes, want 78`);
  if (rank !== 8) throw new Error(`demo-primes: rank ${rank}, want 8`);
  if (types.length !== LIMIT + 1) throw new Error(`demo-primes: ${types.length} types, want ${LIMIT + 1}`);
  let struck = 0;
  for (let n = 2; n <= LIMIT; n++) if (types[n] > 1) struck++;
  if (struck !== 321) throw new Error(`demo-primes: ${struck} struck, want 321`);
  const top = Math.max(...types);
  if (top !== 9) throw new Error(`demo-primes: top ${top}, want 9`);
  const ramp = ink.Ramp.tone(ink.dim, ink.orange);
  const grid = new Grid(area, SIDE, SIDE, 0.1);
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const mark = types[row * SIDE + col + 1];
      if (mark === 0) continue;
      grid.fill(pen, col, row, mark === 1 ? ink.yellow : ramp.at((mark - 2) / (top - 2)));
    }
  }
}
