import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

export const units = {};

const LAND: [number, number, number][] = [
  [0.5, 0.44, 0.24],
  [0.3, 0.52, 0.16],
  [0.7, 0.5, 0.17],
  [0.42, 0.6, 0.16],
  [0.62, 0.6, 0.15],
];
const HOMES: [number, number][] = [
  [0.38, 0.34],
  [0.5, 0.34],
  [0.38, 0.46],
];
const TOWER: [number, number] = [0.5, 0.46];
const HOME = 0.09;
const ROUND = 0.02;
const FIELD: [number, number] = [0.63, 0.4];
const FIELDS = 0.1;
const LINE = 0.016;
const TREES: [number, number][] = [
  [0.25, 0.52],
  [0.31, 0.6],
  [0.37, 0.67],
];
const TREE = 0.045;
const SUB: [number, number, number, number] = [0.56, 0.84, 0.32, 0.09];

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "green", 0.16);
  const s = box.w;
  const x = (u: number) => box.x + u * s;
  const y = (v: number) => box.y + v * s;
  for (const [u, v, r] of LAND) pen.disc(x(u), y(v), r * s, paint.light);
  for (const [u, v] of HOMES) pen.round_rect(x(u), y(v), HOME * s, HOME * s, ROUND * s, paint.white);
  pen.round_rect(x(TOWER[0]), y(TOWER[1] - HOME / 2), HOME * s, HOME * s * 1.5, ROUND * s, paint.white);
  pen.rect(x(FIELD[0]), y(FIELD[1]), FIELDS * s, FIELDS * s * 1.4, paint.deep);
  for (let k = 1; k < 4; k++) {
    const row = y(FIELD[1]) + (k * FIELDS * s * 1.4) / 4;
    pen.segment([x(FIELD[0]) + LINE * s, row], [x(FIELD[0] + FIELDS) - LINE * s, row], LINE * s, paint.white);
  }
  for (const [u, v] of TREES) pen.disc(x(u), y(v), TREE * s, paint.white);
  const [u, v, w, h] = SUB;
  pen.round_rect(x(u), y(v), w * s, h * s, (h * s) / 2, paint.white);
  pen.round_rect(x(u + w * 0.3), y(v - h * 0.8), w * s * 0.26, h * s, ROUND * s, paint.white);
}
