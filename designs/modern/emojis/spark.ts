import type { Pen } from "@mrlyprod/draw";
import { AMBER, GOLD, LIGHT } from "../own/palette.ts";
import { sparkle } from "../own/shape.ts";

const STARS = [
  [12.4, 18.6, 11],
  [24.4, 7.6, 5.6],
  [25.4, 24.4, 4],
] as const;

export default function draw(pen: Pen) {
  for (const [x, y, s] of STARS) {
    pen.polygon(sparkle(x, y, s), AMBER);
    pen.polygon(sparkle(x, y - s * 0.06, s * 0.92), GOLD);
    pen.polygon(sparkle(x, y - s * 0.08, s * 0.4), LIGHT);
  }
}
