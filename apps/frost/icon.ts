import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";
import { EMPTY, grow, parent, SEED } from "./engine.ts";

const SIDE = 15;
const AGES = 7;
const LINE = 0.33;
const CORE = 0.55;
const BUD = 12.5;
const PETAL = 0.5;
const HEART = 0.32;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "cyan", 0.2);
  const c = box.w / SIDE;
  const mid = (SIDE * SIDE - 1) / 2;
  let board = `${EMPTY.repeat(mid)}${SEED}${EMPTY.repeat(mid)}`;
  for (let i = 0; i < AGES; i++) board = grow(SIDE, board);
  const at = (p: number): [number, number] => [
    box.x + ((p % SIDE) + 0.5) * c,
    box.y + (Math.floor(p / SIDE) + 0.5) * c,
  ];
  for (let p = 0; p < board.length; p++) {
    const up = parent(SIDE, p, board[p]);
    if (up >= 0) pen.segment(at(up), at(p), c * LINE, paint.white);
  }
  pen.disc(...at(mid), c * CORE, paint.white);
  const x = box.x + BUD * c;
  const y = box.y + BUD * c;
  for (let i = 0; i < 5; i++) {
    const a = -Math.PI / 2 + (i * 2 * Math.PI) / 5;
    pen.disc(x + Math.cos(a) * c * PETAL, y + Math.sin(a) * c * PETAL, c * PETAL, paint.white);
  }
  pen.disc(x, y, c * HEART, paint.tone);
}
