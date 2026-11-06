import type { Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "......bb....rr..",
  "...o.......rr...",
  "...o..k...rr..o.",
  ".....kok..r.....",
  ".....kkok....mm.",
  "....krrkok..mm..",
  "....krrrkok.....",
  "...kyyrrrkok..b.",
  "...kyyyrrrkok.b.",
  "...kryyyrrkk..e.",
  "..krrryykk....e.",
  "..kyrkkk........",
  ".kykk...........",
  ".kk.............",
  "................",
];

export default function draw(pen: Pen) {
  stamp(pen, ROWS);
}
