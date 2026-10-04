import * as num from "mrlyjs/num";
import { Grid, type Ink, type Pen } from "mrlyjs/view";

export const units = { num };

const N = 100;

export default function draw(pen: Pen, ink: Ink) {
  const grid = new Grid(pen.frame(0.08), N, N, 0);
  let lit = 0;
  let deepest = 1;
  for (let row = 0; row < N; row++) {
    for (let col = 0; col < N; col++) {
      const layer = Number(num.factor.gcd(col + 1, N - row));
      if (layer === 1) {
        lit++;
        grid.fill(pen, col, row, ink.blue);
        continue;
      }
      deepest = Math.max(deepest, layer);
      const [x, y, w, h] = grid.cell(col, row);
      const side = w / layer;
      pen.rect(x + (w - side) / 2, y + (h - side) / 2, side, side, ink.dim);
    }
  }
  if (BigInt(lit) !== num.lattice.coprime_pairs(N)) throw new Error(`wiki-visible-lattice-points: ${lit} lit, want ${num.lattice.coprime_pairs(N)}`);
  if (lit !== 6087) throw new Error(`wiki-visible-lattice-points: ${lit} lit, want 6087`);
  if (deepest !== N) throw new Error(`wiki-visible-lattice-points: deepest layer ${deepest}, want ${N}`);
}
