import type { Color, Pen } from "@mrlyprod/draw";
import { loop } from "../own/icon.ts";
import { STAR } from "../own/star.ts";

export default function draw(pen: Pen, ink: Color) {
  loop(pen, ink, STAR);
}
