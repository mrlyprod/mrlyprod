import type { Color, Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  ".......##.......",
  "......#yy#......",
  "......#yy#......",
  ".....#yyyy#.....",
  ".....#ykky#.....",
  "....#yykkyy#....",
  "....#yykkyy#....",
  "...#yyykkyyy#...",
  "...#yyykkyyy#...",
  "..#yyyyyyyyyy#..",
  "..#yyyykkyyyy#..",
  ".#yyyyykkyyyyy#.",
  ".#yyyyyyyyyyyy#.",
  ".##############.",
  "................",
];

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, ROWS, ink);
}
