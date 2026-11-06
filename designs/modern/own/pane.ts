import type { Color, Pen } from "@mrlyprod/draw";
import { line, loop } from "./icon.ts";
import { box } from "./path.ts";

export function pane(pen: Pen, ink: Color, right: boolean) {
  const x = right ? 14.6 : 9.4;
  loop(pen, ink, box(3.4, 4.9, 17.2, 14.2, 3));
  line(pen, ink, [
    [x, 4.9],
    [x, 19.1],
  ]);
}
