import * as core from "mrlyjs/core";
import * as math from "mrlyjs/math";
import { iso } from "mrlyjs/view";
import type { Pen } from "mrlyjs/view";

export const units = { core, math };

const SIDE = 27;
const FILLED = 8000;
const CUT = 1;
const IN = 2;

type Quad = ReturnType<typeof math.three.quads>[number];

let memo: Quad[] | undefined;

function quads() {
  if (memo) return memo;
  const sponge = math.three.create(23, 3, 3, 2);
  if (sponge.shape[2] !== SIDE) throw new Error(`demo-crop: sponge width ${sponge.shape[2]}, want ${SIDE}`);
  if (math.three.fills(sponge) !== FILLED) throw new Error(`demo-crop: sponge fills ${math.three.fills(sponge)}, want ${FILLED}`);
  const types = { shape: sponge.shape, data: sponge.types };
  const cut = math.shape.named("octahedron", 3, new math.shape.Frac(1, 2));
  const tally = math.shape.census(cut, types);
  const kept = math.cell.models.new_(math.shape.crop(types, cut, true));
  const fills = math.three.fills(kept);
  if (fills !== tally.filled[IN] + tally.filled[CUT]) throw new Error(`demo-crop: kept ${fills}, want ${tally.filled[IN] + tally.filled[CUT]}`);
  if (fills >= FILLED) throw new Error(`demo-crop: kept ${fills}, want under ${FILLED}`);
  memo = math.three.quads(kept);
  return memo;
}

export default function draw(pen: Pen) {
  iso.draw(pen, pen.frame(0.08), quads(), core.colors.shades(core.colors.ORANGE()), null);
}
