import type { Color, Pen } from "@mrlyprod/draw";
import { Grid } from "@mrlyprod/draw";
import * as math from "@mrlyprod/math";
import { plate } from "../../net/sdk/plate.ts";
import { boot, grown, HOME, lay } from "./engine.ts";

export const units = { math };

const NUMBER = 3;
const LEVEL = 3;
const SEED = 7;
const MESS = 6;
const GAP = 0.1;
const CORNER = 0.22;
const GHOST: Color = [255, 255, 255, 80];

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "orange", 0.16);
  const motif = grown(math, NUMBER, 1, HOME);
  const goal = grown(math, NUMBER, LEVEL, HOME);
  const state = boot({ number: NUMBER, level: LEVEL, motif, goal, mess: MESS, seed: SEED });
  if (state.over) throw new Error("mend: the figure deal is already mended");
  const { side, cells } = lay(state);
  const grid = new Grid(box, side, side, GAP);
  for (let i = 0; i < cells.length; i++) {
    if (!cells[i] && goal[i] !== "1") continue;
    const [x, y, w, h] = grid.cell(i % side, Math.floor(i / side));
    pen.round_rect(x, y, w, h, w * CORNER, cells[i] ? paint.white : GHOST);
  }
}
