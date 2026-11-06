import type { Color, Pen, Point } from "@mrlyprod/draw";
import { CONE, HALF, MID, slice, TILT } from "../own/cone.ts";
import { BLUE, GOLD, GREEN, MOUTH, PINK, PURPLE, RED } from "../own/palette.ts";
import { oval, smooth } from "../own/path.ts";

const STREAMERS: [Point[], Color][] = [
  [
    [
      [15.6, 14.6],
      [16.4, 10.6],
      [14.4, 7.4],
      [16.4, 3.4],
    ],
    BLUE,
  ],
  [
    [
      [18.4, 16.4],
      [22.2, 12.8],
      [25.2, 13.6],
      [27.4, 9.8],
    ],
    PINK,
  ],
  [
    [
      [17.6, 19.6],
      [21.4, 20.4],
      [24, 24],
      [27.6, 23.4],
    ],
    GREEN,
  ],
];

const BITS: [number, number, number, Color][] = [
  [21.6, 4.4, 1.3, RED],
  [27, 5.2, 1.1, GOLD],
  [9.6, 7.2, 1.2, PURPLE],
  [27.6, 17.2, 1.2, BLUE],
  [22.6, 28, 1.2, PURPLE],
];

export default function draw(pen: Pen) {
  for (const [pts, color] of STREAMERS) pen.polyline(smooth(pts, false), 2, color);
  for (const [x, y, r, color] of BITS) pen.disc(x, y, r, color);
  pen.polygon(CONE, GOLD);
  pen.polygon(oval(MID[0], MID[1], HALF, 2.4, TILT), GOLD);
  for (const [t0, t1] of [
    [4.6, 7],
    [9.6, 12],
  ])
    pen.polygon(slice(t0, t1), RED);
  pen.polygon(oval(MID[0], MID[1], HALF - 1, 1.6, TILT), MOUTH);
}
