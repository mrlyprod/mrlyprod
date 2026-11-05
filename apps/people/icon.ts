import type { Color, Frame, Pen, Point } from "@mrlyprod/draw";
import { mix } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const STEPS = 24;

type Body = { x: number; head: number; r: number; base: number; rx: number; ry: number };

const BACK: Body = { x: 0.34, head: 0.3, r: 0.12, base: 0.78, rx: 0.25, ry: 0.27 };

const FRONT: Body = { x: 0.6, head: 0.4, r: 0.15, base: 1, rx: 0.32, ry: 0.33 };

function person(pen: Pen, box: Frame, one: Body, color: Color) {
  const at = (u: number, v: number): Point => [box.x + u * box.w, box.y + v * box.h];
  pen.disc(...at(one.x, one.head), one.r * box.w, color);
  const dome = Array.from({ length: STEPS + 1 }, (_, i) => {
    const turn = Math.PI + (Math.PI * i) / STEPS;
    return at(one.x + one.rx * Math.cos(turn), one.base + one.ry * Math.sin(turn));
  });
  pen.polygon(dome, color);
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "orange", 0.2);
  person(pen, box, BACK, mix(paint.white, paint.tone, 0.35));
  person(pen, box, FRONT, paint.white);
}
