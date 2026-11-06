import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "...##......##...",
  ".##############.",
  ".#rrrrrrrrrrrr#.",
  ".#rrrrrrrrrrrr#.",
  ".#rrrrrrrrrrrr#.",
  ".#wwwwwwwwwwww#.",
  ".#wkkwwkkwwkkw#.",
  ".#wkkwwkkwwkkw#.",
  ".#wwwwwwwwwwww#.",
  ".#wwwwwwwwwwww#.",
  ".#wkkwwkkwwkkw#.",
  ".#wkkwwkkwwkkw#.",
  ".#wwwwwwwwwwww#.",
  ".##############.",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
