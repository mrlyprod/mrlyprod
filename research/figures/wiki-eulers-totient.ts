import * as num from "mrlyjs/num";
import { plot, type Ink, type Pen, type Point } from "mrlyjs/view";

export const units = { num };

const TOP = 60;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const phi = num.factor.totients(TOP);
  if (phi.length !== TOP + 1) throw new Error(`wiki-eulers-totient: ${phi.length} totients, want ${TOP + 1}`);
  if (phi[59] !== 58n) throw new Error(`wiki-eulers-totient: phi(59) is ${phi[59]}, want 58`);
  const ceiling = TOP - 1;
  const slot = frame.w / TOP;
  const pad = slot * 0.14;
  let touching = 0;
  for (let n = 1; n <= TOP; n++) {
    const value = Number(phi[n]);
    const hit = n > 1 && value === n - 1;
    const prime = num.prime.is_prime(n);
    if (hit !== prime) throw new Error(`wiki-eulers-totient: phi(${n}) is ${value}, prime ${prime}`);
    if (hit) touching++;
    const height = (frame.h * value) / ceiling;
    pen.rect(frame.x + (n - 1) * slot + pad, frame.y + frame.h - height, slot - 2 * pad, height, hit ? ink.yellow : ink.blue);
  }
  if (touching !== 17) throw new Error(`wiki-eulers-totient: ${touching} primes touch the line, want 17`);
  const rise = (n: number): Point => [frame.x + (n - 0.5) * slot, frame.y + frame.h * (1 - (n - 1) / ceiling)];
  pen.segment(rise(1), rise(TOP), 2, ink.line);
  plot.baseline(pen, frame, ink.line);
}
