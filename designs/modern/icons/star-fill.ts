import type { Color, Pen } from "@mrlyprod/draw";
import { loop } from "../own/icon.ts";
import { STAR } from "../own/star.ts";

export default function draw(pen: Pen, ink: Color) {
  pen.polygon(STAR, ink);
  loop(pen, ink, STAR);
}
