import type { Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  ".......k........",
  "......krk.......",
  "......krk...k...",
  ".....krrrk.krk..",
  "..k..krrrk.krk..",
  ".krk.krorkkrrk..",
  ".krrkrroorrrrk..",
  ".krrrroooorrrrk.",
  ".krrooooyoorrrk.",
  ".kroooyyyyoorrk.",
  ".krooyyyyyyoork.",
  ".krooyyyyyyoork.",
  "..kroooyyooork..",
  "...kkkkkkkkkk...",
  "................",
];

export default function draw(pen: Pen) {
  stamp(pen, ROWS);
}
