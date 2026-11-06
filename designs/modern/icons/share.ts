import type { Color, Pen } from "@mrlyprod/draw";
import { line } from "../own/icon.ts";
import { tray } from "../own/tray.ts";

export default function draw(pen: Pen, ink: Color) {
  tray(pen, ink);
  line(pen, ink, [
    [12, 3.4],
    [12, 14.4],
  ]);
  line(pen, ink, [
    [8.8, 6.6],
    [12, 3.4],
    [15.2, 6.6],
  ]);
}
