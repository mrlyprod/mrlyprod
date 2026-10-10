import * as life from "mrlyjs/life";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { life };

const RULE = 110;
const STEPS = 256;

let memo: ArrayLike<number> | undefined;

function diagram() {
  if (memo) return memo;
  const tensor = life.single_seed(RULE, STEPS);
  if (tensor.shape.length !== 2 || tensor.shape[0] !== STEPS + 1 || tensor.shape[1] !== 2 * STEPS + 1) throw new Error(`demo-wolfram: diagram ${tensor.shape}, want ${STEPS + 1},${2 * STEPS + 1}`);
  memo = tensor.data;
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const rows = STEPS + 1;
  const window = 2 * STEPS + 1;
  const cells = diagram();
  let lo = window;
  let hi = 0;
  let live = 0;
  for (let t = 0; t < rows; t++) {
    for (let c = 0; c < window; c++) {
      if (cells[t * window + c] !== 0) {
        lo = Math.min(lo, c);
        hi = Math.max(hi, c);
        live++;
      }
    }
  }
  if (lo !== 0) throw new Error(`demo-wolfram: first live column ${lo}, want 0`);
  if (hi !== STEPS) throw new Error(`demo-wolfram: last live column ${hi}, want ${STEPS}`);
  if (live === 0) throw new Error("demo-wolfram: no live cell");
  const span = hi - lo + 1;
  const scale = Math.max(Math.floor(Math.min(area.w / span, area.h / rows)), 1);
  const ox = Math.round((pen.width - span * scale) / 2);
  const oy = Math.round((pen.height - rows * scale) / 2);
  for (let t = 0; t < rows; t++) {
    for (let c = lo; c <= hi; c++) {
      if (cells[t * window + c] !== 0) pen.rect(ox + (c - lo) * scale, oy + t * scale, scale, scale, ink.fg);
    }
  }
}
