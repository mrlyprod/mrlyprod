import * as core from "mrlyjs/core";
import * as math from "mrlyjs/math";
import { frame as box, iso } from "mrlyjs/view";
import type { Frame, Pen } from "mrlyjs/view";

export const units = { core, math };

const NUMBER = 3;
const SHARE = [0.26, 0.33, 0.4];
const WALK = [0.2, 0.5, 0.8];
const SIDES = [3, 9, 27];
const COUNTS = [20, 200, 4000];

// BLOCKS

function word() {
  return [
    math.bang.MagicLayer.new(new math.name.Bang(23, 3, 2), NUMBER),
    math.bang.MagicLayer.new(new math.name.Bang(9, 3, 2), NUMBER),
    math.bang.MagicLayer.new(new math.name.Bang(23, 3, 2), NUMBER),
  ];
}

function block(k: number) {
  const letters = word();
  if (k === 1) return math.three.create(letters[0].design.code, NUMBER, 1, 2);
  return math.cell.models.new_(math.bang.magic(letters.slice(0, k)));
}

function stack(frame: Frame) {
  return SHARE.map((share, i) => {
    const side = frame.w * share;
    const cx = frame.x + frame.w * WALK[i];
    const cy = frame.y + frame.h * WALK[i];
    return box(cx - side / 2, cy - side / 2, side, side);
  });
}

// DRAW

export default function draw(pen: Pen) {
  const frame = pen.frame(0.08);
  const shade = core.colors.shades(core.colors.ORANGE());
  stack(frame).forEach((panel, k) => {
    const cell = block(k + 1);
    if (cell.shape.length !== 3 || cell.shape.some((n) => n !== SIDES[k])) throw new Error(`demo-tower: block ${k + 1} is ${cell.shape}, want ${SIDES[k]} cubed`);
    const fills = math.three.fills(cell);
    if (fills !== COUNTS[k]) throw new Error(`demo-tower: block ${k + 1} fills ${fills}, want ${COUNTS[k]}`);
    iso.draw(pen, panel, math.three.quads(cell), shade, null);
  });
}
