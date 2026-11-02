import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

const LEVELS = 3;
const GROUND = 0.94;
const ROOT = 0.68;
const ROWS = [0.68, 0.46, 0.3, 0.16];
const SPREAD = 0.92;
const BARK = 0.05;
const THIN = 0.7;
const SMALL = 0.028;
const BIG = 0.056;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "pink", 0.16);
  const at = (d: number, p: number): [number, number] => [
    box.x + box.w * ((1 - SPREAD) / 2 + ((p + 0.5) * SPREAD) / 2 ** d),
    box.y + box.h * ROWS[d],
  ];
  pen.polyline(
    [
      [box.x + box.w / 2, box.y + box.h * GROUND],
      [box.x + box.w / 2, box.y + box.h * ROOT],
    ],
    BARK * box.w * 1.2,
    paint.white,
  );
  for (let d = 1; d <= LEVELS; d++)
    for (let p = 0; p < 2 ** d; p++) pen.polyline([at(d - 1, p >> 1), at(d, p)], BARK * box.w * THIN ** d, paint.white);
  const leaves = 2 ** LEVELS;
  for (let p = 0; p < leaves; p++) {
    const [x, y] = at(LEVELS, p);
    pen.disc(x, y, (SMALL + ((BIG - SMALL) * p) / (leaves - 1)) * box.w, p < leaves / 2 ? paint.light : paint.white);
  }
}
