import type { Pen } from "@mrlyprod/draw";
import { CORE, EMBER, FLAME } from "../own/palette.ts";
import { curve, toward } from "../own/shape.ts";

const OUTER = curve(
  [15, 1.6],
  [
    [18.6, 6],
    [21.4, 9.6],
  ],
  [
    [22.6, 8],
    [24, 5.2],
  ],
  [
    [29.2, 12],
    [28.2, 19.6],
  ],
  [
    [27.4, 30.4],
    [16, 30.4],
  ],
  [
    [4.6, 30.4],
    [3.8, 19.6],
  ],
  [
    [3.4, 12.6],
    [8, 8.6],
  ],
  [
    [8.4, 11.6],
    [10.6, 11],
  ],
  [
    [10.4, 5.6],
    [15, 1.6],
  ],
);

const INNER = curve(
  [16.4, 12.4],
  [
    [22.8, 18.6],
    [22.2, 24],
  ],
  [
    [21.4, 29.4],
    [16, 29.4],
  ],
  [
    [10.6, 29.4],
    [10, 24],
  ],
  [
    [10, 18.6],
    [16.4, 12.4],
  ],
);

export default function draw(pen: Pen) {
  pen.polygon(OUTER, EMBER);
  pen.polygon(toward(OUTER, 0.8), FLAME);
  pen.polygon(INNER, CORE);
}
