import type { Color, Pen } from "@mrlyprod/draw";
import { TRAY } from "../own/shapes.ts";
import { stamp } from "../own/stamp.ts";

const ARROW = [
  "................",
  ".......##.......",
  "......####......",
  ".....######.....",
  "....########....",
  ".......##.......",
  ".......##.......",
  ".......##.......",
  ".......##.......",
  ".......##.......",
  ".......##.......",
  ".......##.......",
  "................",
  "................",
  "................",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, TRAY, ink);
  stamp(pen, ARROW, ink);
}
