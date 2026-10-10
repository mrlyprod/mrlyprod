import * as math from "mrlyjs/math";
import { field, frame, type Ink, type Pen } from "mrlyjs/view";

const SCALES = 20;
const SHEET = 1075;
const START = 215;
const SIZE = 860;

export const units = { math };

let memo: { values: Float64Array; lo: number } | undefined;

function crop() {
  if (memo) return memo;
  const numbers = Array.from({ length: SCALES }, (_, i) => i + 1);
  const sheet = math.moire.stack(math.moire.Spec.new(495, 3, 2), numbers, "Sum", 1, "Square", SHEET, []);
  if (Math.trunc(sheet.max()) !== SCALES) throw new Error(`site-research: sheet peaks at ${sheet.max()}, want ${SCALES}`);
  if (sheet.size !== SHEET) throw new Error(`site-research: sheet of ${sheet.size}, want ${SHEET}`);
  const data = sheet.data;
  const values = new Float64Array(SIZE * SIZE);
  for (let row = 0; row < SIZE; row++) {
    const base = (START + row) * sheet.size + START;
    for (let col = 0; col < SIZE; col++) values[row * SIZE + col] = data[base + col];
  }
  sheet.free();
  let lo = Number.MAX_VALUE;
  for (const v of values) lo = Math.min(lo, v);
  if (!values.includes(SCALES)) throw new Error(`site-research: no sample reaches ${SCALES}`);
  memo = { values, lo };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { values, lo } = crop();
  const hi = SCALES;
  const levels = Math.trunc(hi - lo) + 1;
  const stops = Array.from({ length: levels - 1 }, (_, i) => {
    const t = i / (levels - 2);
    return ink.mix(ink.ground, ink.blue, 0.2 + 0.8 * t ** 2.1);
  });
  stops.push(ink.yellow);
  const edge = (pen.width - SIZE) / 2;
  field.draw_range(pen, frame(edge, edge, SIZE, SIZE), SIZE, SIZE, values, [lo, hi], new ink.Ramp(stops));
}
