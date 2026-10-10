import * as math from "mrlyjs/math";
import { hex } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const CODES = [0, 1, 3, 7, 23, 31, 63, 127, 255];
const SIDE = 11;
const WIDE = 3;

type Slice = ReturnType<typeof math.six.cut_design>;

let memo: Slice[] | undefined;

function slices() {
  if (memo) return memo;
  let last = 0;
  const cuts = CODES.map((code, corners) => {
    const held = math.bang.code_to_corners(code, 3, 2).length;
    if (held !== corners) throw new Error(`demo-spectrometer: code ${code} has ${held} corners, want ${corners}`);
    if (corners > 0 && (code & CODES[corners - 1]) !== CODES[corners - 1]) throw new Error(`demo-spectrometer: code ${code} does not hold code ${CODES[corners - 1]}`);
    const slice = math.six.cut_design(code, SIDE, 1, 2);
    const fills = math.six.fills(slice);
    if (slice.cell.shape.length !== 2 || slice.cell.shape[0] !== 2 * SIDE || slice.cell.shape[1] < hex.row_len(SIDE, SIDE - 1)) throw new Error(`demo-spectrometer: the cut of code ${code} is ${slice.cell.shape}`);
    if (fills < last) throw new Error(`demo-spectrometer: code ${code} fills ${fills}, below ${last}`);
    last = fills;
    return slice;
  });
  const full = math.six.fills(cuts[8]);
  if (full !== 726) throw new Error(`demo-spectrometer: the last cut fills ${full}, want 726`);
  if (!cuts.some((slice) => math.six.fills(slice) > 0)) throw new Error("demo-spectrometer: no cut holds a fill");
  memo = cuts;
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const FILL = math.six.FILL();
  const frame = pen.frame(0.08);
  const stage = frame.panels(WIDE, WIDE, (frame.w / WIDE) * 0.05);
  if (hex.count(SIDE) !== 726) throw new Error(`demo-spectrometer: ${hex.count(SIDE)} triangles, want 726`);
  slices().forEach((slice, corners) => {
    const panel = stage[corners];
    const edge = panel.w / (2 * SIDE);
    hex.hexagon(pen, panel, SIDE, 0, () => ink.line);
    hex.hexagon(pen, panel, SIDE, edge * 0.06, (row, col) => (hex.at(slice, SIDE, row, col) !== FILL ? ink.ground : null));
    hex.hexagon(pen, panel, SIDE, edge * 0.06, (row, col) => (hex.at(slice, SIDE, row, col) === FILL ? ink.blue : null));
  });
}
