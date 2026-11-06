import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  ".....######.....",
  "....#wwwwww#....",
  "...#wwwnnwww#...",
  "..#wwwwnnwwww#..",
  ".#wwwwwwwwwwww#.",
  ".#wwwwnnnwwwww#.",
  ".#wwwwwnnwwwww#.",
  ".#wwwwwnnwwwww#.",
  ".#wwwwwnnwwwww#.",
  ".#wwwwwnnwwwww#.",
  "..#wwwnnnnwww#..",
  "...#wwwwwwww#...",
  "....#wwwwww#....",
  ".....######.....",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
