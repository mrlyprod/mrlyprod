import { field, frame, type Ink, type Pen } from "mrlyjs/view";
import census from "./census/demo-modes.json" with { type: "json" };

const SPAN = 243;
const CELL = 3;
const HALF = (SPAN - 1) / 2;
const WEDGE = ((HALF + 1) * (HALF + 2)) / 2;

export const units = {};

function reads() {
  const wedge = census.reads;
  if (wedge.length !== WEDGE) throw new Error(`demo-modes: ${wedge.length} reads, want ${WEDGE}`);
  if (wedge.some((v) => !(v >= 0 && v <= 1))) throw new Error("demo-modes: a read off the unit interval");
  const values = new Float64Array(SPAN * SPAN);
  for (let row = 0; row < SPAN; row++) {
    for (let col = 0; col < SPAN; col++) {
      const a = Math.abs(row - HALF);
      const b = Math.abs(col - HALF);
      const hi = Math.max(a, b);
      const lo = Math.min(a, b);
      values[row * SPAN + col] = wedge[(hi * (hi + 1)) / 2 + lo];
    }
  }
  return values;
}

export default function draw(pen: Pen, ink: Ink) {
  const ramp = new ink.Ramp([ink.ground, ink.blue, ink.yellow]);
  const plate = CELL * SPAN;
  const edge = Math.floor((pen.width - plate) / 2);
  field.draw_range(pen, frame(edge, edge, plate, plate), SPAN, SPAN, reads(), [0, 1], ramp);
}
