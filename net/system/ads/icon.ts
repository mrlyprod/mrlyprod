import type { Pen, Point } from "@mrlyprod/draw";
import { mix } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

const EDGE = 204.8;

const SIDE = 614.4;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "purple", 0.2);
  const p = (x: number, y: number): Point => [box.x + ((x - EDGE) / SIDE) * box.w, box.y + ((y - EDGE) / SIDE) * box.h];
  const s = (n: number) => (n / SIDE) * box.w;
  const rect = (x: number, y: number, w: number, h: number, r: number, color = paint.white) =>
    pen.round_rect(...p(x, y), s(w), s(h), s(r), color);
  rect(196, 404, 168, 216, 36, mix(paint.white, paint.tone, 0.4));
  pen.polygon([p(332, 404), p(808, 220), p(808, 804), p(332, 620)], paint.white);
  rect(372, 600, 104, 216, 40, mix(paint.white, paint.tone, 0.4));
}
