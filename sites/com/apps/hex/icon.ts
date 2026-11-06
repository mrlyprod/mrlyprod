import type { Color, Pen } from "@mrlyprod/draw";
import { hex as comb } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";
import * as hex from "@mrlyprod/hex";
import type { CellHex } from "@mrlyprod/math";

export const units = { hex };

const CODE = "23";
const NUMBER = 3;
const LEVEL = 2;
const BASE = 2;
const SIDE = 9;
const WANT = { fills: 306, voids: 180 };
const INSET = 0.16;
const GAP = 2;
const GHOST = 45;

let slice: CellHex | null = null;

function facts() {
  if (slice) return slice;
  const cut = hex.cut_design(CODE, NUMBER, LEVEL, BASE);
  const tally = hex.census(cut, false);
  if (tally.triangles !== comb.count(SIDE))
    throw new Error(`hex: the cut holds ${tally.triangles} triangles, want ${comb.count(SIDE)}`);
  if (tally.fills !== WANT.fills || tally.voids !== WANT.voids)
    throw new Error(
      `hex: the carpet's cut fills ${tally.fills} and leaves ${tally.voids}, want ${WANT.fills} and ${WANT.voids}`,
    );
  slice = cut;
  return slice;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "teal", INSET);
  const [r, g, b] = paint.white;
  const tones: Partial<Record<number, Color>> = {
    [hex.FILL()]: paint.white,
    [hex.VOID()]: [r, g, b, GHOST],
  };
  comb.draw(pen, box, facts(), GAP, (type) => tones[type] ?? null);
}
