import type { Color, Pen } from "@mrlyprod/draw";
import { line, loop } from "../own/icon.ts";
import { round } from "../own/path.ts";
import { pencil } from "../own/pencil.ts";

const H = 2.7;

export default function draw(pen: Pen, ink: Color) {
  loop(
    pen,
    ink,
    pencil(
      round(
        [
          [-10.2, 0],
          [-4.2, -H],
          [10, -H],
          [10, H],
          [-4.2, H],
        ],
        [0.8, 0, 1.6, 1.6, 0],
        true,
      ),
    ),
  );
  line(
    pen,
    ink,
    pencil([
      [6, -H],
      [6, H],
    ]),
  );
}
