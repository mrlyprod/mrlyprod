import * as math from "mrlyjs/math";
import { hex } from "mrlyjs/view";

export const units = { math };

const CODE = "23";
const NUMBER = 3;
const LEVEL = 2;
const BASE = 2;
const SIDE = 9;
const WANT = { fills: 306, voids: 180 };
const MARGIN = 0.08;
const GAP = 2;
const FAINT = 0.22;

let slice = null;

function facts() {
  if (slice) return slice;
  const cut = math.six.cut_design(CODE, NUMBER, LEVEL, BASE);
  const tally = math.six.census(cut, false);
  if (tally.triangles !== hex.count(SIDE)) throw new Error(`six: the cut holds ${tally.triangles} triangles, want ${hex.count(SIDE)}`);
  if (tally.fills !== WANT.fills || tally.voids !== WANT.voids) throw new Error(`six: the carpet's cut fills ${tally.fills} and leaves ${tally.voids}, want ${WANT.fills} and ${WANT.voids}`);
  slice = cut;
  return slice;
}

export default function draw(pen, ink) {
  const tones = { [math.six.FILL()]: ink.blue, [math.six.VOID()]: ink.mix(ink.ground, ink.blue, FAINT) };
  hex.draw(pen, pen.frame(MARGIN), facts(), GAP, (type) => tones[type] ?? null);
}
