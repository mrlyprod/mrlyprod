import * as core from "mrlyjs/core";
import * as math from "mrlyjs/math";
import { frame, iso } from "mrlyjs/view";
import type { Frame, Ink, Pen } from "mrlyjs/view";

export const units = { core, math };

const CODE = 127;
const NUMBER = 2;
const LEVEL = 4;

type Quad = ReturnType<typeof math.three.quads>[number];

let memo: { seed: Quad[]; solid: Quad[] } | undefined;

function quads() {
  if (memo) return memo;
  const seed = math.three.create(CODE, NUMBER, 1, 2);
  const solid = math.three.create(CODE, NUMBER, LEVEL, 2);
  if (math.three.fills(seed) !== 7) throw new Error(`demo-sponge: seed fills ${math.three.fills(seed)}, want 7`);
  if (math.three.fills(solid) !== 7 ** LEVEL) throw new Error(`demo-sponge: solid fills ${math.three.fills(solid)}, want ${7 ** LEVEL}`);
  if (solid.shape.length !== 3 || solid.shape.some((n) => n !== 16)) throw new Error(`demo-sponge: solid is ${solid.shape}, want 16 cubed`);
  memo = { seed: math.three.quads(seed), solid: math.three.quads(solid) };
  return memo;
}

function corner(area: Frame, size: number, right: boolean) {
  const side = area.w * size;
  const [x, y] = right ? [area.x + area.w - side, area.y + area.h - side] : [area.x, area.y];
  return frame(x, y, side, side);
}

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const shade = core.colors.shades(core.colors.GREEN());
  const { seed, solid } = quads();
  iso.draw(pen, corner(area, 0.4, false), seed, shade, ink.ground);
  iso.draw(pen, corner(area, 0.64, true), solid, shade, null);
}
