import type { Pen, Point } from "@mrlyprod/draw";
import { FACE, GLOW, RIM } from "../own/palette.ts";
import { shift, smooth } from "../own/path.ts";

const EDGE = 1;
const DROP = 0.7;
const PALM = smooth([
  [7.6, 14.2],
  [12.8, 12.4],
  [16.6, 17.6],
  [16, 27.4],
  [11, 28.6],
  [7.4, 27.2],
  [6.6, 20.6],
]);
const FINGERS = [
  [14.2, 24.6],
  [18.2, 24.6],
  [22.2, 24.2],
  [26, 23.2],
] as const;
const THUMB: [Point, Point] = [
  [11.2, 16.4],
  [12.8, 6.4],
];

export default function draw(pen: Pen) {
  for (const dy of [0, DROP]) {
    pen.polygon(shift(PALM, 0, dy), RIM);
    pen.polyline(shift([...PALM, PALM[0]!], 0, dy), EDGE, RIM);
    pen.segment(...(shift(THUMB, 0, dy) as [Point, Point]), 7.4 + EDGE, RIM);
    for (const [y, x] of FINGERS) pen.segment([14.2, y + dy], [x, y + dy], 4.4 + EDGE, RIM);
  }
  pen.polygon(PALM, FACE);
  for (const [y, x] of FINGERS) {
    for (const dy of [0, DROP]) pen.segment([14.2, y + dy], [x, y + dy], 4.4 + EDGE, RIM);
    pen.segment([14.2, y], [x, y], 4.4, FACE);
  }
  pen.segment(...THUMB, 7.4, FACE);
  pen.segment([10.4, 8.4], [11.2, 14.6], 1.8, GLOW);
  pen.round_rect(3, 14.4, 4.2, 14.4, 1.4, RIM);
}
