import * as math from "mrlyjs/math";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const ROWS = 64;
const ODD = 729;

export default function draw(pen: Pen, ink: Ink) {
  const gasket = math.two.create(7, 2, 6, 0, 2);
  if (gasket.shape[0] !== ROWS || gasket.shape[1] !== ROWS) throw new Error(`wiki-pascals-triangle: gasket ${gasket.shape}, want ${ROWS} by ${ROWS}`);
  const frame = pen.frame(0.06);
  const step = frame.w / ROWS;
  const [cx] = frame.center();
  let odd = 0;
  for (let n = 0; n < ROWS; n++) {
    const y = frame.y + (n + 0.5) * step;
    for (let k = 0; k <= n; k++) {
      const x = cx + (k - n / 2) * step;
      if (gasket.types[(n - k) * ROWS + k] !== 0) {
        pen.disc(x, y, step * 0.42, ink.blue);
        odd++;
      } else {
        pen.disc(x, y, step * 0.11, ink.dim);
      }
    }
  }
  const fills = math.two.fills(gasket);
  if (odd !== ODD || fills !== ODD) throw new Error(`wiki-pascals-triangle: ${odd} odd discs of ${fills} fills, want ${ODD}`);
}
