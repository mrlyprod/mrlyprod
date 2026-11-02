import type { Pen, Point } from "@mrlyprod/draw";
import { mix } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

const EDGE = 204.8;

const SIDE = 614.4;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "green", 0.2);
  const p = (x: number, y: number): Point => [box.x + ((x - EDGE) / SIDE) * box.w, box.y + ((y - EDGE) / SIDE) * box.h];
  const s = (n: number) => (n / SIDE) * box.w;
  const rect = (x: number, y: number, w: number, h: number, r: number, color = paint.white) =>
    pen.round_rect(...p(x, y), s(w), s(h), s(r), color);
  rect(196, 236, 520, 160, 56, mix(paint.white, paint.tone, 0.4));
  rect(196, 320, 632, 456, 72);
  rect(604, 480, 224, 136, 48, paint.tone);
}
