import * as math from "@mrlyprod/mrly/math";
import type { Color, Pen, Tensor } from "#view";
import { Grid } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = { math };

const CODE = 7;
const BASE = 2;
const NUMBER = 3;
const LEVEL = 3;
const RADIUS = [1, 2];
const WANT = { side: 27, in: 332, cut: 76 };

let held: Tensor | null = null;

function crop() {
  if (held) return held;
  const tile = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  const r = new math.shape.Frac(RADIUS[0], RADIUS[1]);
  const ball = math.shape.named("ball", 2, r);
  r.free();
  const map = math.shape.regions(ball, tile.shape).data;
  const { filled } = math.shape.census(ball, { shape: tile.shape, data: tile.types });
  if (tile.shape[0] !== WANT.side || filled[2] !== WANT.in || filled[1] !== WANT.cut)
    throw new Error(
      `crop: a side of ${tile.shape[0]} keeps ${filled[2]} in and ${filled[1]} cut, want ${WANT.side}, ${WANT.in} and ${WANT.cut}`,
    );
  held = { shape: tile.shape, data: tile.types.map((on, i) => (on ? map[i] + 1 : 0)) };
  return held;
}

export default function draw(pen: Pen) {
  const { box, paint, size } = plate(pen, "mint", 0.17);
  const cells = crop();
  const [rows, cols] = cells.shape;
  const grid = new Grid(box, cols, rows, 0);
  const tones: (Color | null)[] = [null, [255, 255, 255, 56], [255, 255, 255, 170], paint.white];
  grid.paint(pen, cells, (type) => tones[type]);
  const [cx, cy] = box.center();
  pen.ring(cx, cy, box.w / 2, size * 0.014, paint.deep);
}
