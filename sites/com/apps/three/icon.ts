import type { Pen } from "@mrlyprod/draw";
import { iso } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";
import * as three from "@mrlyprod/three";

export const units = { three };

const LEVEL = 2;
const WANT = { fills: 20 ** LEVEL };

let memo: three.Quad[] | undefined;

function quads(): three.Quad[] {
  if (memo) return memo;
  const sponge = three.create(23, 3, LEVEL, 2);
  const fills = three.fills(sponge);
  if (fills !== WANT.fills) throw new Error(`three: sponge fills ${fills}, want ${WANT.fills}`);
  memo = three.quads(sponge);
  const surface = Number(three.surface(sponge));
  if (memo.length !== surface) throw new Error(`three: ${memo.length} quads, the sponge shows ${surface} faces`);
  return memo;
}

export default function draw(pen: Pen): void {
  const { box, paint } = plate(pen, "blue", 0.16);
  iso.draw(pen, box, quads(), [paint.white, paint.light, paint.deep], null);
}
