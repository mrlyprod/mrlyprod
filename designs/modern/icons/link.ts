import type { Color, Pen, Point } from "@mrlyprod/draw";
import { line } from "../own/icon.ts";
import { arc, turn } from "../own/path.ts";

const R = 3.4;
const IN = 2;
const OUT = 6.4;

const HALF: Point[] = [
  ...arc(12 + IN, 12, R, Math.PI + 0.25, Math.PI / 2),
  ...arc(12 + OUT, 12, R, Math.PI / 2, -Math.PI / 2),
  [12 + IN + 1.6, 12 - R],
];

export default function draw(pen: Pen, ink: Color) {
  line(pen, ink, turn(HALF, -Math.PI / 4));
  line(pen, ink, turn(HALF, (3 * Math.PI) / 4));
}
