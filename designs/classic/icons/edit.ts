import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "............##..",
  "...........#rr#.",
  "..........#rrr#.",
  ".........#ssr#..",
  "........#yys#...",
  ".......#yyy#....",
  "......#yyy#.....",
  ".....#yyy#......",
  "....#yyy#.......",
  "...#wyy#........",
  "..#www#.........",
  "..#ww#..........",
  ".#kk#...........",
  ".##.............",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
