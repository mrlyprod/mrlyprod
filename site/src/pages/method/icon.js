import { plate } from "../../lib/plate.js";

export const units = {};

const SIDE = 0.32;
const ROUND = 0.24;
const STROKE = 0.09;
const STOPS = [
  [SIDE / 2, SIDE / 2],
  [0.5, 0.5],
  [1 - SIDE / 2, 1 - SIDE / 2],
];

function elbow(a, b) {
  return [a, [b[0], a[1]], b];
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "mint", 0.18);
  const side = box.w * SIDE;
  const thick = box.w * STROKE;
  for (let i = 0; i + 1 < STOPS.length; i++) {
    const [a, b] = [STOPS[i], STOPS[i + 1]];
    if (a[0] !== a[1] || b[0] !== b[1] || b[0] - a[0] <= SIDE)
      throw new Error(`method: stops ${a} and ${b} are not on the diagonal with a visible elbow between them`);
    pen.polyline(elbow(box.at(...a), box.at(...b)), thick, paint.deep);
  }
  for (const [u, v] of STOPS) {
    const [x, y] = box.at(u, v);
    pen.round_rect(x - side / 2, y - side / 2, side, side, side * ROUND, paint.white);
  }
}
