import * as math from "mrlyjs/math";
import { iso } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const CODE = 23;
const NUMBER = 3;
const LEVEL = 3;
const BASE = 2;
const FILLS = 20;
const MARGIN = 0.08;
const SIDES = [0.72, 0.5];

let quads: math.three.Quad[] | null = null;

function facts() {
  if (quads) return quads;
  const cell = math.three.create(CODE, NUMBER, LEVEL, BASE);
  const read = math.three.census(cell);
  if (read.fills !== FILLS ** LEVEL) throw new Error(`app-three: the sponge fills ${read.fills} cells, want ${FILLS ** LEVEL}`);
  quads = math.three.quads(cell);
  if (quads.length !== Number(read.surface)) throw new Error(`app-three: ${quads.length} quads, the census says ${read.surface} exposed faces`);
  return quads;
}

export default function draw(pen: Pen, ink: Ink) {
  const shade = [ink.blue, ...SIDES.map((k) => ink.mix(ink.ground, ink.blue, k))];
  iso.draw(pen, pen.frame(MARGIN), facts(), shade);
}
