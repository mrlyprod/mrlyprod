import * as math from "mrlyjs/math";
import { iso } from "mrlyjs/view";
import { plate } from "../../lib/icon.js";

export const units = { math };

const LEVEL = 3;
const WANT = { fills: 20 ** LEVEL };

let memo;

function quads() {
  if (memo) return memo;
  const sponge = math.three.create(23, 3, LEVEL, 2);
  const fills = math.three.fills(sponge);
  if (fills !== WANT.fills) throw new Error(`home: sponge fills ${fills}, want ${WANT.fills}`);
  memo = math.three.quads(sponge);
  return memo;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "blue", 0.16);
  iso.draw(pen, box, quads(), [paint.white, paint.light, paint.deep], null);
}
