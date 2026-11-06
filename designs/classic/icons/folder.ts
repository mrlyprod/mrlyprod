import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "................",
  ".#####..........",
  ".#yyyy#.........",
  ".##############.",
  ".#yyyyyyyyyyyy#.",
  ".#yyyyyyyyyyyy#.",
  ".#yyyyyyyyyyyy#.",
  ".#yyyyyyyyyyyy#.",
  ".#yyyyyyyyyyyy#.",
  ".#yyyyyyyyyyyy#.",
  ".#yyyyyyyyyyyy#.",
  ".#yyyyyyyyyyyy#.",
  ".##############.",
  "................",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
