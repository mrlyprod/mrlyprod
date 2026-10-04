import * as core from "mrlyjs/core";
import * as math from "mrlyjs/math";
import { iso } from "mrlyjs/view";
import type { Pen } from "mrlyjs/view";

export const units = { core, math };

const CODE = 23;
const SIZE = 64;
const SCALES = [1, 3, 5, 7, 9, 11];
const FILLED = 34416;

type Quad = ReturnType<typeof math.three.quads>[number];

let memo: Quad[] | undefined;

function quads() {
  if (memo) return memo;
  const field = math.moire.volume(math.moire.Spec.new(CODE, 2, 3), SCALES, "Sum", 1, SIZE);
  const mark = SCALES.length;
  if (field.data.length !== SIZE ** 3) throw new Error(`demo-volume: ${field.data.length} samples, want ${SIZE ** 3}`);
  if (field.max() !== mark) throw new Error(`demo-volume: peak ${field.max()}, want ${mark}`);
  if (field.count(mark) !== FILLED) throw new Error(`demo-volume: ${field.count(mark)} at the peak, want ${FILLED}`);
  const shell = math.cell.models.new_(field.solid(mark));
  if (math.three.fills(shell) !== FILLED) throw new Error(`demo-volume: shell fills ${math.three.fills(shell)}, want ${FILLED}`);
  memo = math.three.quads(shell);
  return memo;
}

export default function draw(pen: Pen) {
  iso.draw(pen, pen.frame(0.08), quads(), core.colors.shades(core.colors.INDIGO()), null);
}
