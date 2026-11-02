import type { Color, Pen, Point } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

export const units = {};

const POSTS = [0.1, 0.36, 0.64, 0.9];
const TOP = 0.42;
const FOOT = 0.78;
const RAILS = [0.52, 0.66];
const POST = 0.06;
const RAIL = 0.035;
const FALLEN: [Point, Point][] = [
  [
    [0.38, 0.8],
    [0.64, 0.71],
  ],
];
const STEM: [Point, Point] = [
  [0.5, 0.66],
  [0.5, 0.3],
];
const LEAVES: [number, number, number][] = [
  [0.44, 0.4, 0.05],
  [0.56, 0.36, 0.05],
];
const FRUIT: [number, number, number] = [0.5, 0.24, 0.065];

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "green", 0.16);
  const s = box.w;
  const at = ([x, y]: Point): Point => [box.x + s * x, box.y + box.h * y];
  const soft: Color = [255, 255, 255, 150];
  pen.segment(at(STEM[0]), at(STEM[1]), s * RAIL, paint.white);
  for (const [x, y, r] of LEAVES) pen.disc(...at([x, y]), s * r, paint.white);
  pen.disc(...at([FRUIT[0], FRUIT[1]]), s * FRUIT[2], paint.light);
  for (const [a, b] of [
    [0, 1],
    [2, 3],
  ])
    for (const y of RAILS) pen.segment(at([POSTS[a], y]), at([POSTS[b], y]), s * RAIL, paint.white);
  for (const [a, b] of FALLEN) pen.segment(at(a), at(b), s * RAIL, soft);
  for (const x of POSTS) pen.segment(at([x, TOP]), at([x, FOOT]), s * POST, paint.white);
}
