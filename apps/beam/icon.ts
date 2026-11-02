import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

const PATH: [number, number][] = [
  [0, 6],
  [2, 6],
  [2, 2],
  [5, 2],
  [5, 5],
  [7, 5],
];
const STAR: [number, number] = [7, 5];
const SIDE = 8;
const BEAM = 0.36;
const GLASS = 0.5;
const PANE = 0.22;
const LAMP = 0.55;
const RAY = 0.85;
const PIT = 0.2;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "orange", 0.2);
  const c = box.w / SIDE;
  const at = ([x, y]: [number, number]): [number, number] => [box.x + (x + 0.5) * c, box.y + y * c];
  pen.polyline(PATH.map(at), c * BEAM, paint.white);
  for (const [x, y] of PATH.slice(1, -1)) {
    const [cx, cy] = at([x, y]);
    const lean = x === 5 ? -1 : 1;
    const d = c * GLASS;
    pen.segment([cx - d, cy + lean * d], [cx + d, cy - lean * d], c * PANE, paint.white);
  }
  pen.disc(...at(PATH[0]), c * LAMP, paint.white);
  const [sx, sy] = at(STAR);
  const r = c * RAY;
  const k = r * PIT;
  pen.polygon(
    [
      [sx, sy - r],
      [sx + k, sy - k],
      [sx + r, sy],
      [sx + k, sy + k],
      [sx, sy + r],
      [sx - k, sy + k],
      [sx - r, sy],
      [sx - k, sy - k],
    ],
    paint.white,
  );
}
