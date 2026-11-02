import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "purple", 0.2);
  const w = box.w;
  const h = box.h;
  const screen = h * 0.68;
  pen.round_rect(box.x, box.y + h * 0.04, w, screen, w * 0.09, paint.white);
  pen.round_rect(box.x + w * 0.06, box.y + h * 0.1, w * 0.88, screen - h * 0.12, w * 0.05, paint.tone);
  pen.round_rect(box.x + w * 0.28, box.y + h * 0.82, w * 0.44, h * 0.08, h * 0.04, paint.white);
  const cx = box.x + w * 0.52;
  const cy = box.y + h * 0.04 + screen / 2;
  pen.triangle([cx - w * 0.1, cy - h * 0.14], [cx + w * 0.14, cy], [cx - w * 0.1, cy + h * 0.14], paint.white);
}
