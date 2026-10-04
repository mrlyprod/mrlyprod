import * as life from "mrlyjs/life";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { life };

const RULE = 90;
const STEPS = 63;

let memo: ArrayLike<number> | undefined;

function diagram() {
  if (memo) return memo;
  const tensor = life.single_seed(RULE, STEPS);
  if (tensor.shape.length !== 2 || tensor.shape[0] !== STEPS + 1 || tensor.shape[1] !== 2 * STEPS + 1) throw new Error(`wiki-cellular-automaton: diagram ${tensor.shape}, want ${STEPS + 1},${2 * STEPS + 1}`);
  memo = tensor.data;
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const rows = STEPS + 1;
  const window = 2 * STEPS + 1;
  const cells = diagram();
  let live = 0;
  for (let t = 0; t < rows; t++) {
    for (let c = 0; c < window; c++) {
      if (cells[t * window + c] !== 0) live++;
    }
  }
  if (live !== 729) throw new Error(`wiki-cellular-automaton: ${live} live cells, want 729`);
  const scale = Math.min(area.w / window, area.h / rows);
  const ox = (pen.width - window * scale) / 2;
  const oy = (pen.height - rows * scale) / 2;
  const gap = scale * 0.12;
  for (let t = 0; t < rows; t++) {
    for (let c = 0; c < window; c++) {
      if (cells[t * window + c] !== 0) pen.rect(ox + c * scale + gap, oy + t * scale + gap, scale - 2 * gap, scale - 2 * gap, ink.yellow);
    }
  }
}
