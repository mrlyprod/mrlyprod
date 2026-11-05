import type { Color, Frame, Pen, Point } from "@mrlyprod/draw";
import { mix } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const EDGE = 204.8;

const SIDE = 614.4;

type Heart = { x: number; y: number; k: number };

const BACK: Heart = { x: 384, y: 384, k: 0.6 };

const FRONT: Heart = { x: 576, y: 590, k: 0.8 };

function heart(pen: Pen, box: Frame, one: Heart, color: Color) {
  const at = (u: number, v: number): Point => [
    box.x + ((one.x + u * one.k - EDGE) / SIDE) * box.w,
    box.y + ((one.y + v * one.k - EDGE) / SIDE) * box.h,
  ];
  const r = ((176 * one.k) / SIDE) * box.w;
  pen.disc(...at(-140, -118), r, color);
  pen.disc(...at(140, -118), r, color);
  pen.polygon([at(-308, -58), at(308, -58), at(0, 288)], color);
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "purple", 0.2);
  heart(pen, box, BACK, mix(paint.white, paint.tone, 0.4));
  heart(pen, box, FRONT, paint.white);
}
