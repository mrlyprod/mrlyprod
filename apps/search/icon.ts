import type { Pen } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "cyan", 0.2);
  const c = box.w * 0.4;
  pen.disc(box.x + c, box.y + c, box.w * 0.36, paint.white);
  pen.disc(box.x + c, box.y + c, box.w * 0.23, paint.tone);
  pen.polygon(
    [
      [box.x + box.w * 0.64, box.y + box.h * 0.72],
      [box.x + box.w * 0.72, box.y + box.h * 0.64],
      [box.x + box.w, box.y + box.h * 0.92],
      [box.x + box.w * 0.92, box.y + box.h],
    ],
    paint.white,
  );
}
