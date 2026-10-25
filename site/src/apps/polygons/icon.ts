import type { Pen, Point } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = {};

const R3 = Math.sqrt(3);
const E = 1 / (1 + R3);
const AP = (E * R3) / 2;
const GAP = 0.055;
const INSET = 0.17;

const at = (deg: number, r: number, s = 0): Point => {
  const a = (deg * Math.PI) / 180;
  return [r * Math.cos(a) - s * Math.sin(a), r * Math.sin(a) + s * Math.cos(a)];
};

const HEX = Array.from({ length: 6 }, (_, k) => at(30 + 60 * k, E));
const square = (deg: number) => [
  at(deg, AP, -E / 2),
  at(deg, AP + E, -E / 2),
  at(deg, AP + E, E / 2),
  at(deg, AP, E / 2),
];
const triangle = (k: number) => [HEX[k], at(60 * k, AP + E, E / 2), at(60 * k + 60, AP + E, -E / 2)];
const TILES = [HEX, square(0), square(60), square(120), triangle(0), triangle(1)];
const SQUARES = Array.from({ length: 6 }, (_, k) => square(60 * k));
const TRIANGLES = Array.from({ length: 6 }, (_, k) => triangle(k));
const REACH = Math.hypot(AP + E, E / 2);

const area = (pts: Point[]) =>
  Math.abs(
    pts.reduce((sum, [x, y], i) => sum + x * pts[(i + 1) % pts.length][1] - pts[(i + 1) % pts.length][0] * y, 0),
  ) / 2;

const centre = (pts: Point[]): Point => [
  pts.reduce((s, p) => s + p[0], 0) / pts.length,
  pts.reduce((s, p) => s + p[1], 0) / pts.length,
];

const shrink = (pts: Point[], by: number) => {
  const [cx, cy] = centre(pts);
  const r = Math.min(
    ...pts.map(([x, y], i) => {
      const [nx, ny] = pts[(i + 1) % pts.length];
      return Math.abs((nx - x) * (y - cy) - (x - cx) * (ny - y)) / Math.hypot(nx - x, ny - y);
    }),
  );
  const k = Math.max(r - by, 0) / r;
  return pts.map(([x, y]): Point => [cx + (x - cx) * k, cy + (y - cy) * k]);
};

function check() {
  const kinds = TILES.map((t) => t.length);
  if (kinds.join() !== "6,4,4,4,3,3")
    throw new Error("polygons: a cell of 3.4.6.4 is one hexagon, three squares and two triangles");
  const whole = TILES.reduce((sum, t) => sum + area(t), 0);
  if (Math.abs(whole - R3 / 2) > 1e-9)
    throw new Error(`polygons: the tiles cover ${whole}, and the lattice cell is ${R3 / 2}`);
  const rosette = [HEX, ...SQUARES, ...TRIANGLES].reduce((sum, t) => sum + area(t), 0);
  if (Math.abs(rosette - E * E * (6 + 3 * R3)) > 1e-9)
    throw new Error(`polygons: the rosette covers ${rosette}, want ${E * E * (6 + 3 * R3)}`);
}

export default function draw(pen: Pen) {
  check();
  const { box, paint } = plate(pen, "cyan", INSET);
  const [cx, cy] = box.center();
  const s = box.radius() / REACH;
  const place = (tile: Point[]) =>
    shrink(
      tile.map(([x, y]): Point => [cx + x * s, cy + y * s]),
      (GAP * s * E) / 2,
    );
  for (const tile of TRIANGLES) pen.polygon(place(tile), paint.deep);
  for (const tile of SQUARES) pen.polygon(place(tile), paint.light);
  pen.polygon(place(HEX), paint.white);
}
