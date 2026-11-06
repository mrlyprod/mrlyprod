import type { Color, Pen } from "@mrlyprod/draw";
import { line } from "../own/icon.ts";
import { tray } from "../own/tray.ts";

export default function draw(pen: Pen, ink: Color) {
  tray(pen, ink);
  line(pen, ink, [
    [12, 3.4],
    [12, 16],
  ]);
  line(pen, ink, [
    [8.8, 12.8],
    [12, 16],
    [15.2, 12.8],
  ]);
}
