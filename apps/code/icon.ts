import type { Pen } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "purple", 0.2);
  const mid = box.y + box.h / 2;
  const t = box.w * 0.08;
  pen.polygon(
    [
      [box.x + box.w * 0.3, box.y + box.h * 0.2],
      [box.x, mid],
      [box.x + box.w * 0.3, box.y + box.h * 0.8],
      [box.x + box.w * 0.3 + t, box.y + box.h * 0.72],
      [box.x + t * 2, mid],
      [box.x + box.w * 0.3 + t, box.y + box.h * 0.28],
    ],
    paint.white,
  );
  pen.polygon(
    [
      [box.x + box.w * 0.7, box.y + box.h * 0.2],
      [box.x + box.w, mid],
      [box.x + box.w * 0.7, box.y + box.h * 0.8],
      [box.x + box.w * 0.7 - t, box.y + box.h * 0.72],
      [box.x + box.w - t * 2, mid],
      [box.x + box.w * 0.7 - t, box.y + box.h * 0.28],
    ],
    paint.white,
  );
  pen.polygon(
    [
      [box.x + box.w * 0.56, box.y + box.h * 0.1],
      [box.x + box.w * 0.66, box.y + box.h * 0.1],
      [box.x + box.w * 0.44, box.y + box.h * 0.9],
      [box.x + box.w * 0.34, box.y + box.h * 0.9],
    ],
    paint.white,
  );
}
