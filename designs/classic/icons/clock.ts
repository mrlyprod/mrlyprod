import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  ".....######.....",
  "....#wwwwww#....",
  "...#wwwwwwww#...",
  "..#wwwwkkwwww#..",
  ".#wwwwwkkwwwww#.",
  ".#wwwwwkkwwwww#.",
  ".#wwwwwkkkkwww#.",
  ".#wwwwwkkkkwww#.",
  ".#wwwwwwwwwwww#.",
  ".#wwwwwwwwwwww#.",
  "..#wwwwwwwwww#..",
  "...#wwwwwwww#...",
  "....#wwwwww#....",
  ".....######.....",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
