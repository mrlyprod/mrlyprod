import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "....########....",
  "..##wwwwwwww##..",
  ".#wwwwwwwwwwww#.",
  ".#wwwwwwwwwwww#.",
  ".##wwwwwwwwww##.",
  ".#b##########b#.",
  ".#bbbbbbbbbbbb#.",
  ".#bbbbbbbbbbbb#.",
  ".##bbbbbbbbbb##.",
  ".#b##########b#.",
  ".#bbbbbbbbbbbb#.",
  ".#bbbbbbbbbbbb#.",
  "..##bbbbbbbb##..",
  "....########....",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
