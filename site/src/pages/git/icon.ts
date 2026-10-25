import type { Pen } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = {};

const STROKE = 0.11;
const RISE = 0.34;
const BRACKET = 0.22;
const SLASH = 0.1;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "gray", 0.16);
  const [cx, cy] = box.center();
  const w = box.w;
  const thick = w * STROKE;
  const h = w * RISE;
  const tip = w / 2 - thick / 2;
  const heel = tip - w * BRACKET;
  const lean = w * SLASH;
  if (heel <= lean + thick)
    throw new Error(
      `git: the brackets overlap the slash, heel ${heel.toFixed(0)}, lean ${lean.toFixed(0)}, stroke ${thick.toFixed(0)}`,
    );
  pen.polyline(
    [
      [cx - heel, cy - h],
      [cx - tip, cy],
      [cx - heel, cy + h],
    ],
    thick,
    paint.white,
  );
  pen.polyline(
    [
      [cx + heel, cy - h],
      [cx + tip, cy],
      [cx + heel, cy + h],
    ],
    thick,
    paint.white,
  );
  pen.segment([cx + lean, cy - h], [cx - lean, cy + h], thick, paint.light);
}
