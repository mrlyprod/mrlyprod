import type { Pen } from "@mrlyprod/draw";
import { iso } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

const LEVEL = 3;
const SIDE = 3 ** LEVEL;
const WANT = { fills: 20 ** LEVEL };

let memo: iso.Quad[] | undefined;

function filled(x: number, y: number, z: number) {
  if (x < 0 || y < 0 || z < 0 || x >= SIDE || y >= SIDE || z >= SIDE) return false;
  for (let a = x, b = y, c = z; a || b || c; a = (a / 3) | 0, b = (b / 3) | 0, c = (c / 3) | 0)
    if ((a % 3 === 1 ? 1 : 0) + (b % 3 === 1 ? 1 : 0) + (c % 3 === 1 ? 1 : 0) > 1) return false;
  return true;
}

function quads() {
  if (memo) return memo;
  const out: iso.Quad[] = [];
  let fills = 0;
  const unit = (n: number) => Math.fround((2 * n) / SIDE - 1);
  const at = (x: number, y: number, z: number) => ({ x: unit(x), y: unit(y), z: unit(z) });
  for (let x = 0; x < SIDE; x++)
    for (let y = 0; y < SIDE; y++)
      for (let z = 0; z < SIDE; z++) {
        if (!filled(x, y, z)) continue;
        fills++;
        if (!filled(x + 1, y, z))
          out.push({
            normal: { x: 1, y: 0, z: 0 },
            verts: [at(x + 1, y, z), at(x + 1, y + 1, z), at(x + 1, y + 1, z + 1), at(x + 1, y, z + 1)],
          });
        if (!filled(x, y + 1, z))
          out.push({
            normal: { x: 0, y: 1, z: 0 },
            verts: [at(x, y + 1, z), at(x, y + 1, z + 1), at(x + 1, y + 1, z + 1), at(x + 1, y + 1, z)],
          });
        if (!filled(x, y, z + 1))
          out.push({
            normal: { x: 0, y: 0, z: 1 },
            verts: [at(x, y, z + 1), at(x + 1, y, z + 1), at(x + 1, y + 1, z + 1), at(x, y + 1, z + 1)],
          });
      }
  if (fills !== WANT.fills) throw new Error(`home: sponge fills ${fills}, want ${WANT.fills}`);
  memo = out;
  return memo;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "blue", 0.16);
  iso.draw(pen, box, quads(), [paint.white, paint.light, paint.deep], null);
}
