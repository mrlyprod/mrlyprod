import * as math from "mrlyjs/math";
import { Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const CODE = 7;
const BASE = 2;
const NUMBER = 3;
const LEVEL = 2;
const COPIES = 5;
const RADIUS = [1, 2];
const MARGIN = 0.08;
const FADE = 0.35;
const WANT = { side: 45, lit: 1144 };

type Sheet = { shape: number[]; types: Uint8Array; kept: Uint8Array };

let sheet: Sheet | null = null;

function facts(): Sheet {
  if (sheet) return sheet;
  const tile = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  const block = math.two.merge(Array.from({ length: COPIES * COPIES }, () => tile), COPIES, COPIES);
  const radius = new math.shape.Frac(RADIUS[0], RADIUS[1]);
  const ball = math.shape.named("ball", 2, radius);
  radius.free();
  const tensor = { shape: block.shape, data: block.types };
  const kept = math.shape.crop(tensor, ball, false);
  const inside = math.shape.census(ball, tensor).filled[2];
  const lit = math.two.fills({ shape: kept.shape, types: kept.data });
  if (block.shape[0] !== WANT.side || lit !== inside || lit !== WANT.lit) throw new Error(`app-two: a sheet of ${block.shape[0]} lights ${lit} inside the circle, the census says ${inside}, want ${WANT.side} and ${WANT.lit}`);
  sheet = { shape: block.shape, types: block.types as Uint8Array, kept: kept.data as Uint8Array };
  return sheet;
}

export default function draw(pen: Pen, ink: Ink) {
  const { shape, types, kept } = facts();
  const [rows, cols] = shape;
  const cells = new Grid(pen.frame(MARGIN), cols, rows, 0);
  const faded = ink.mix(ink.ground, ink.dim, FADE);
  cells.paint(pen, { shape, types }, (type) => (type ? faded : null));
  cells.paint(pen, { shape, types: kept }, (type) => (type ? ink.blue : null));
}
