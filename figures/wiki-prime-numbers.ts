import * as num from "mrlyjs/num";
import { Grid, type Ink, type Pen } from "mrlyjs/view";

export const units = { num };

const LIMIT = 100;
const SIDE = 10;

export default function draw(pen: Pen, ink: Ink) {
  const sieve = new num.prime.Sieve(LIMIT);
  sieve.finish();
  const [count, rank, types] = [sieve.count(), sieve.rank(), sieve.types()];
  sieve.free();
  if (count !== 25) throw new Error(`wiki-prime-numbers: ${count} primes, want 25`);
  if (rank !== 4) throw new Error(`wiki-prime-numbers: rank ${rank}, want 4`);
  if (types.length !== LIMIT + 1) throw new Error(`wiki-prime-numbers: ${types.length} types, want ${LIMIT + 1}`);
  let struck = 0;
  for (let n = 2; n <= LIMIT; n++) if (types[n] > 1) struck++;
  if (struck !== 74) throw new Error(`wiki-prime-numbers: ${struck} struck, want 74`);
  const top = Math.max(...types);
  if (top !== 5) throw new Error(`wiki-prime-numbers: top mark ${top}, want 5`);
  const ramp = ink.Ramp.tone(ink.dim, ink.orange);
  const grid = new Grid(pen.frame(0.08), SIDE, SIDE, 0.1);
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const mark = types[row * SIDE + col + 1];
      if (mark === 0) continue;
      grid.fill(pen, col, row, mark === 1 ? ink.yellow : ramp.at((mark - 2) / (top - 2)));
    }
  }
}
