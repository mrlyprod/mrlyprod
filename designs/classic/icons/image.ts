import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "................",
  ".##############.",
  ".#bbbbbbbbbbbb#.",
  ".#bbbbbbbbyybb#.",
  ".#bbbbbbbyyyyb#.",
  ".#bbbbbbbyyyyb#.",
  ".#bbbbbbbbyybb#.",
  ".#bbbbeebbbbbb#.",
  ".#bbbeeeebbbbb#.",
  ".#bbeeeeeebbee#.",
  ".#beeeeeeeeeee#.",
  ".#eeeeeeeeeeee#.",
  ".##############.",
  "................",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
