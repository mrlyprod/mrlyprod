import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "teal", 0.2);
  const w = box.w;
  const h = box.h;
  const cx = box.x + w / 2;
  const thick = w * 0.09;
  pen.round_rect(cx - w * 0.16, box.y, w * 0.32, h * 0.56, w * 0.16, paint.white);
  pen.arc([cx, box.y + h * 0.4], w * 0.34, [Math.PI * 0.15, Math.PI * 0.85], thick, paint.white);
  pen.rect(cx - thick / 2, box.y + h * 0.74, thick, h * 0.14, paint.white);
  pen.round_rect(cx - w * 0.22, box.y + h * 0.88, w * 0.44, h * 0.1, h * 0.05, paint.white);
}
