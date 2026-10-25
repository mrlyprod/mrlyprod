import * as life from "@mrlyprod/mrly/life";
import type { Pen } from "#view";
import { Grid } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = { life };

const RULE = 110;
const STEPS = 15;
const GAP = 0.14;
const CORNER = 0.22;

type Sheet = { side: number; types: Uint8Array };

let sheet: Sheet | null = null;

function cone(): Sheet {
  if (sheet) return sheet;
  const made = life.single_seed(RULE, STEPS);
  const [rows, cols] = made.shape;
  const side = STEPS + 1;
  const types = new Uint8Array(side * side);
  let all = 0;
  let kept = 0;
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      const bit = made.data[r * cols + c];
      all += bit;
      if (c < side) {
        types[r * side + c] = bit;
        kept += bit;
      }
    }
  }
  if (rows !== side || kept !== all || !types[side - 1])
    throw new Error(
      `wolfram: rule ${RULE} from one cell keeps ${kept} of ${all} live cells in the left half of its cone over ${rows} rows, want all of them over ${side}`,
    );
  sheet = { side, types };
  return sheet;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "yellow", 0.16);
  const { side, types } = cone();
  const cells = new Grid(box, side, side, GAP);
  for (let r = 0; r < side; r++) {
    for (let c = 0; c < side; c++) {
      if (!types[r * side + c]) continue;
      const [x, y, w, h] = cells.cell(c, r);
      pen.round_rect(x, y, w, h, w * CORNER, r === 0 ? paint.light : paint.white);
    }
  }
}
