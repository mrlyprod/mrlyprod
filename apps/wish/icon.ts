import type { Color, Pen } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

export const units = {};

const BIG = 0.3;
const INNER = 0.11;
const UP = 0.08;
const OVER = 0.05;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "blue", 0.16);
  const [cx, cy] = box.center();
  const s = box.w;
  const far: Color = [255, 255, 255, 170];
  const near: Color = [255, 255, 255, 205];
  pen.disc(cx - s * 0.34, cy + s * 0.4, s * 0.035, far);
  pen.disc(cx - s * 0.25, cy + s * 0.3, s * 0.06, near);
  pen.disc(cx + s * OVER, cy - s * UP, s * BIG, paint.white);
  pen.disc(cx + s * OVER, cy - s * UP, s * INNER, paint.light);
}
