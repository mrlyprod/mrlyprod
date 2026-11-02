import type { Color, Pen } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

export const units = {};

const COIN = 0.22;
const FACE = 0.14;
const RIM = 0.022;
const NEAR = 0.17;
const DOTS = 9;
const REACH = 0.44;
const DOT = 0.018;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "purple", 0.16);
  const [cx, cy] = box.center();
  const s = box.w;
  const far: Color = [255, 255, 255, 115];
  const line: Color = [255, 255, 255, 170];
  for (let i = 0; i < DOTS; i++) {
    const k = (i / (DOTS - 1) - 0.5) * 2 * REACH;
    pen.disc(cx + s * k, cy + s * k, s * DOT, line);
  }
  pen.disc(cx + s * NEAR, cy - s * NEAR, s * COIN, far);
  pen.disc(cx - s * NEAR, cy + s * NEAR, s * COIN, paint.white);
  pen.disc(cx - s * NEAR, cy + s * NEAR, s * (FACE + RIM), paint.light);
  pen.disc(cx - s * NEAR, cy + s * NEAR, s * (FACE - RIM), paint.white);
}
