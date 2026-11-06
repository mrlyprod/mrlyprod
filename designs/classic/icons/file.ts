import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "...#######......",
  "...#wwwww##.....",
  "...#wwwww#s#....",
  "...#wwwww####...",
  "...#wwwwwwww#...",
  "...#wwwwwwww#...",
  "...#wssssssw#...",
  "...#wwwwwwww#...",
  "...#wssssssw#...",
  "...#wwwwwwww#...",
  "...#wssssssw#...",
  "...#wwwwwwww#...",
  "...##########...",
  "................",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
