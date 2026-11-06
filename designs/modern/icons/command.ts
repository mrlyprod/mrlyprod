import type { Color, Pen, Point } from "@mrlyprod/draw";
import { loop } from "../own/icon.ts";
import { arc, turn } from "../own/path.ts";

const LO = 9.2;
const HI = 14.8;
const R = 2.5;

const QUARTER: Point[] = [[LO, LO - R], [LO, HI + R], ...arc(LO - R, HI + R, R, 0, (3 * Math.PI) / 2).slice(1)];

export default function draw(pen: Pen, ink: Color) {
  loop(
    pen,
    ink,
    [0, 1, 2, 3].flatMap((k) => turn(QUARTER, (-k * Math.PI) / 2)),
  );
}
