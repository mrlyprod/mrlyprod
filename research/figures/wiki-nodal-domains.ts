import { field, frame, type Ink, type Pen } from "mrlyjs/view";
import census from "./census/wiki-nodal-domains.json" with { type: "json" };

const RES = 256;
const REACH = 2;
const PANELS = 4;

export const units = {};

// FIELD

let memo: Float64Array[] | null = null;

function signs(rows: number[][]) {
  if (rows.length !== RES) throw new Error(`wiki-nodal-domains: ${rows.length} rows, want ${RES}`);
  const out = new Int8Array(RES * RES);
  rows.forEach(([first, ...flips], row) => {
    if (first !== 0 && first !== 1) throw new Error(`wiki-nodal-domains: row ${row} opens on ${first}, want 0 or 1`);
    let sign = first === 1 ? 1 : -1;
    let at = 0;
    for (const flip of flips) {
      if (!(flip > at && flip < RES)) throw new Error(`wiki-nodal-domains: row ${row} flips at ${flip} after ${at}`);
      out.fill(sign, row * RES + at, row * RES + flip);
      sign = -sign;
      at = flip;
    }
    out.fill(sign, row * RES + at, (row + 1) * RES);
  });
  return out;
}

function nodal(sign: Int8Array) {
  const values = new Float64Array(RES * RES);
  for (let row = 0; row < RES; row++) {
    for (let col = 0; col < RES; col++) {
      const i = row * RES + col;
      const r1 = Math.min(row + REACH + 1, RES);
      const c1 = Math.min(col + REACH + 1, RES);
      let crossed = false;
      for (let r = Math.max(row - REACH, 0); r < r1 && !crossed; r++) {
        for (let c = Math.max(col - REACH, 0); c < c1; c++) {
          if (sign[r * RES + c] !== sign[i]) {
            crossed = true;
            break;
          }
        }
      }
      values[i] = crossed ? 0 : sign[i];
    }
  }
  return values;
}

function fields() {
  if (memo) return memo;
  const { panels } = census;
  if (panels.length !== PANELS) throw new Error(`wiki-nodal-domains: ${panels.length} panels, want ${PANELS}`);
  memo = panels.map((rows) => nodal(signs(rows)));
  return memo;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const box = pen.frame(0.08);
  const gap = box.w * 0.06;
  const side = (box.w - gap) / 2;
  const boxes = [
    frame(box.x, box.y, side, side),
    frame(box.x + side + gap, box.y, side, side),
    frame(box.x, box.y + side + gap, side, side),
    frame(box.x + side + gap, box.y + side + gap, side, side),
  ];
  const ramp = new ink.Ramp([ink.orange, ink.ground, ink.blue]);
  const values = fields();
  boxes.forEach((panel, i) => field.draw_range(pen, panel, RES, RES, values[i], [-1, 1], ramp));
}
