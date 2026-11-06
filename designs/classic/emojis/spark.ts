import type { Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "............k...",
  "...........kyk..",
  "......k...kyyyk.",
  ".....kyk...kyk..",
  ".....kyk....k...",
  ".....kyk........",
  "..kkkyyykkk.....",
  ".kyyyyyyyyyk....",
  "..kkkyyykkk.....",
  ".....kyk....k...",
  ".....kyk...kyk..",
  ".....kyk..kyyyk.",
  "......k....kyk..",
  "............k...",
  "................",
];

export default function draw(pen: Pen) {
  stamp(pen, ROWS);
}
