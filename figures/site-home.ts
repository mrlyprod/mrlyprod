import * as math from "mrlyjs/math";
import { iso } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

type Quad = ReturnType<typeof math.three.quads>[number];

let memo: Quad[] | undefined;

function quads() {
  if (memo) return memo;
  const sponge = math.three.create(23, 3, 3, 2);
  if (math.three.fills(sponge) !== 8000) throw new Error(`site-home: sponge fills ${math.three.fills(sponge)}, want 8000`);
  memo = math.three.quads(sponge);
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  iso.draw(pen, pen.frame(0.08), quads(), [ink.fg, ink.blue, ink.dim], null);
}
