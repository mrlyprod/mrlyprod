import type { Color, Pen } from "@mrlyprod/draw";
import { pane } from "../own/pane.ts";

export default function draw(pen: Pen, ink: Color) {
  pane(pen, ink, false);
}
