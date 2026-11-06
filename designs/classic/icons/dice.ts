import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "..############..",
  ".#wwwwwwwwwwww#.",
  ".#wkkwwwwwwkkw#.",
  ".#wkkwwwwwwkkw#.",
  ".#wwwwwwwwwwww#.",
  ".#wwwwwwwwwwww#.",
  ".#wwwwwkkwwwww#.",
  ".#wwwwwkkwwwww#.",
  ".#wwwwwwwwwwww#.",
  ".#wwwwwwwwwwww#.",
  ".#wkkwwwwwwkkw#.",
  ".#wkkwwwwwwkkw#.",
  ".#wwwwwwwwwwww#.",
  "..############..",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
