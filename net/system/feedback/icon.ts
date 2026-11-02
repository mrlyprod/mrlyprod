import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

const EDGE = 204.8;

const SIDE = 614.4;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "orange", 0.2);
  const k = box.w / SIDE;
  const p = (x: number, y: number): Point => [box.x + ((x - EDGE) / SIDE) * box.w, box.y + ((y - EDGE) / SIDE) * box.h];
  pen.round_rect(...p(212, 232), 600 * k, 440 * k, 90 * k, paint.white);
  pen.polygon([p(380, 660), p(320, 800), p(520, 660)], paint.white);
  pen.segment(p(512, 330), p(512, 500), 56 * k, paint.tone);
  pen.disc(...p(512, 580), 34 * k, paint.tone);
}
