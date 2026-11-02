import type { Color, Pen } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

export const units = {};

const BODY = 0.31;
const TALL = 0.26;
const EYE = 0.033;
const STEM = 0.016;
const KNOB = 0.039;
const SPARK = 0.021;
const FAR = 0.256;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "pink", 0.16);
  const [cx, cy] = box.center();
  const s = box.w;
  const faint: Color = [255, 255, 255, 170];
  const eye: Color = [255, 50, 90, 140];
  pen.rect(cx - s * (BODY / 2), cy - s * 0.09, s * BODY, s * TALL, paint.white);
  pen.disc(cx - s * 0.06, cy + s * 0.027, s * EYE, eye);
  pen.disc(cx + s * 0.06, cy + s * 0.027, s * EYE, eye);
  pen.segment([cx, cy - s * 0.09], [cx, cy - s * 0.18], s * STEM * 2, paint.white);
  pen.disc(cx, cy - s * 0.19, s * KNOB, paint.white);
  for (const [dx, dy] of [
    [-1, -1],
    [1, -1],
    [-1, 1],
    [1, 1],
  ])
    pen.disc(cx + dx * s * FAR, cy + dy * s * FAR, s * SPARK, faint);
}
