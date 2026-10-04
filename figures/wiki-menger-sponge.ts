import * as math from "mrlyjs/math";
import { iso } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const CODE = 23;
const LEVEL = 3;

type Quad = ReturnType<typeof math.three.quads>[number];

let memo: Quad[] | undefined;

function sponge() {
  if (memo) return memo;
  const cube = math.three.create(CODE, 3, LEVEL, 2);
  const sites = math.three.fills(cube);
  if (sites !== 8000) throw new Error(`wiki-menger-sponge: ${sites} sites, want 8000`);
  memo = math.three.quads(cube);
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  iso.draw(pen, pen.frame(0.08), sponge(), [ink.yellow, ink.orange, ink.dim], null);
}
