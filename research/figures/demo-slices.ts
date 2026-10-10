import * as math from "mrlyjs/math";
import { hex } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const CODE = 23;
const WIDE = 4;

type Slice = ReturnType<typeof math.six.cut_design>;

let memo: { slice: Slice; pieces: number }[] | undefined;

function slices() {
  if (memo) return memo;
  const cuts = [];
  for (let k = 1; k <= WIDE * WIDE; k++) {
    const side = 2 * k - 1;
    const slice = math.six.cut_design(CODE, side, 1, 2);
    const pieces = math.six.components(slice);
    const holes = math.six.holes(slice);
    if (slice.cell.shape.length !== 2 || slice.cell.shape[0] !== 2 * side || slice.cell.shape[1] < hex.row_len(side, side - 1)) throw new Error(`demo-slices: the cut of side ${side} is ${slice.cell.shape}`);
    if (k % 2 === 0) {
      if (pieces !== 1) throw new Error(`demo-slices: side ${side} has ${pieces} pieces, want 1`);
      if (holes === 0) throw new Error(`demo-slices: side ${side} has no holes`);
    } else if (k > 1 && pieces <= 1) throw new Error(`demo-slices: side ${side} has ${pieces} pieces, want more than 1`);
    cuts.push({ slice, pieces });
  }
  memo = cuts;
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const FILL = math.six.FILL();
  const frame = pen.frame(0.08);
  const stage = frame.panels(WIDE, WIDE, (frame.w / WIDE) * 0.05);
  let triangles = 0;
  slices().forEach(({ slice, pieces }, i) => {
    const side = 2 * (i + 1) - 1;
    const color = pieces > 1 ? ink.blue : ink.orange;
    const panel = stage[i];
    const edge = panel.w / (2 * side);
    let painted = 0;
    let offered = 0;
    hex.hexagon(pen, panel, side, edge * 0.14, (row, col) => {
      offered++;
      if (hex.at(slice, side, row, col) !== FILL) return null;
      painted++;
      return color;
    });
    if (offered !== hex.count(side)) throw new Error(`demo-slices: side ${side} offered ${offered}, want ${hex.count(side)}`);
    if (painted !== math.six.fills(slice)) throw new Error(`demo-slices: side ${side} painted ${painted}, want ${math.six.fills(slice)}`);
    triangles += offered;
  });
  if (triangles !== 32736) throw new Error(`demo-slices: ${triangles} triangles, want 32736`);
}
