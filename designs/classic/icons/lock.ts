import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  ".....######.....",
  "....#ssssss#....",
  "...#ss####ss#...",
  "...#s#....#s#...",
  "...#s#....#s#...",
  "..############..",
  "..#yyyyyyyyyy#..",
  "..#yyyyyyyyyy#..",
  "..#yyyykkyyyy#..",
  "..#yyyykkyyyy#..",
  "..#yyyykkyyyy#..",
  "..#yyyyyyyyyy#..",
  "..#yyyyyyyyyy#..",
  "..############..",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
