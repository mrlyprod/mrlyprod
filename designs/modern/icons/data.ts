import type { Color, Pen, Point } from "@mrlyprod/draw";
import { line, loop } from "../own/icon.ts";
import { arc, oval } from "../own/path.ts";

const RX = 7.6;
const RY = 2.8;
const TOP = 6.2;
const LOW = 17.8;

const rim = (y: number): Point[] => arc(0, 0, 1, Math.PI, 0).map(([c, s]): Point => [12 + RX * c, y + RY * s]);

export default function draw(pen: Pen, ink: Color) {
  loop(pen, ink, oval(12, TOP, RX, RY));
  line(pen, ink, [[12 - RX, TOP], ...rim(LOW), [12 + RX, TOP]]);
  line(pen, ink, rim((TOP + LOW) / 2));
}
