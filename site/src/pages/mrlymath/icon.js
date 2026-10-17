import * as math from "@mrlyprod/mrly/math";
import { Grid, frame } from "#view";
import { plate } from "../../lib/plate.js";

export const units = { math };

const CODE = 7;
const BITS = 8;
const SIDE = 27;
const GAP = 0.3;
const WANT = { fills: 512 };

let memo = null;

function facts() {
  if (memo) return memo;
  const corners = math.bang.code_to_corners(CODE, 2, 2).length;
  let lit = 0;
  for (let bit = 0; bit < BITS; bit++) lit += (CODE >> bit) & 1;
  if (lit !== corners) throw new Error(`mrlymath: ${lit} lit bits, want ${corners}`);
  const carpet = math.two.create(CODE, 3, 3, 0, 2);
  if (carpet.shape[0] !== SIDE || carpet.shape[1] !== SIDE) throw new Error(`mrlymath: carpet shape ${carpet.shape}, want ${SIDE} square`);
  let filled = 0;
  for (const kind of carpet.types) filled += kind;
  if (filled !== WANT.fills) throw new Error(`mrlymath: carpet sums to ${filled}, want ${WANT.fills}`);
  memo = carpet;
  return memo;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "indigo", 0.16);
  const carpet = facts();
  const side = box.h / (1 + 1 / BITS + GAP / BITS);
  const pitch = side / BITS;
  const left = box.x + (box.w - side) / 2;
  const row = new Grid(frame(left, box.y, side, pitch), BITS, 1, 0.1);
  for (let bit = 0; bit < BITS; bit++) {
    const [x, y, w, h] = row.cell(bit, 0);
    const on = (CODE >> bit) & 1;
    pen.round_rect(x, y, w, h, w * 0.18, on ? paint.white : paint.deep);
  }
  const top = box.y + box.h - side;
  new Grid(frame(left, top, side, side), SIDE, SIDE, 0).paint(pen, carpet, (kind) => (kind !== 0 ? paint.light : null));
}
