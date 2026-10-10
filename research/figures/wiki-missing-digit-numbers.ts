import * as num from "mrlyjs/num";
import { Grid, type Ink, type Pen } from "mrlyjs/view";

export const units = { num };

const SIDE = 100;
const DIGITS = [0, 1, 2, 3, 4, 5, 6, 8, 9];
const DEPTH = 4;
const KEPT = 6561;
const PRIMES = 680;

export default function draw(pen: Pen, ink: Ink) {
  const top = SIDE * SIDE;
  const prime = num.prime.flags(top);
  const elements = num.design.elements(10, DIGITS, DEPTH);
  if (prime.length !== top + 1) throw new Error(`wiki-missing-digit-numbers: ${prime.length} flags, want ${top + 1}`);
  const kept = [0, ...Array.from(elements, Number)];
  if (kept.some((n, i) => n >= top || (i > 0 && n <= kept[i - 1]))) throw new Error("wiki-missing-digit-numbers: the elements are not ascending below the top");
  if (kept.length !== 9 ** 4) throw new Error(`wiki-missing-digit-numbers: ${kept.length} kept, want ${9 ** 4}`);
  if (kept.length !== KEPT) throw new Error(`wiki-missing-digit-numbers: ${kept.length} kept, want ${KEPT}`);
  const grid = new Grid(pen.frame(0.08), SIDE, SIDE, 0);
  let lit = 0;
  for (const n of kept) {
    if (prime[n]) lit++;
    grid.fill(pen, n % SIDE, Math.floor(n / SIDE), prime[n] ? ink.yellow : ink.blue);
  }
  if (lit !== PRIMES) throw new Error(`wiki-missing-digit-numbers: ${lit} primes, want ${PRIMES}`);
}
