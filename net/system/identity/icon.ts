import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "indigo", 0.2);
  pen.round_rect(box.x, box.y + box.h * 0.12, box.w, box.h * 0.76, box.w * 0.1, paint.white);
  pen.disc(box.x + box.w * 0.28, box.y + box.h * 0.5, box.w * 0.13, paint.tone);
  pen.round_rect(box.x + box.w * 0.5, box.y + box.h * 0.4, box.w * 0.38, box.h * 0.08, box.h * 0.04, paint.tone);
  pen.round_rect(box.x + box.w * 0.5, box.y + box.h * 0.56, box.w * 0.28, box.h * 0.08, box.h * 0.04, paint.tone);
}
