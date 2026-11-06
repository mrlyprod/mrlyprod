import type { Color, Pen } from "@mrlyprod/draw";
import { loop } from "../own/icon.ts";
import { box } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  for (const x of [4, 14]) for (const y of [4, 14]) loop(pen, ink, box(x, y, 6, 6, 1.8));
}
