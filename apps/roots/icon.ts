import type { Color, Pen, Point } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

export const units = {};

const STEPS = 48;
const START = 0.06;
const HALF = 0.34;
const GAP = 0.05;
const SPRING = 0.05;
const BANDS: ((u: number) => number)[] = [(u) => 1 + 1.6 * u * u, () => 1, (u) => 1.1 - 0.2 * u];

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "mint", 0.16);
  const [, cy] = box.center();
  const x0 = box.x + box.w * START;
  const at = (u: number) => {
    const open = ease(u * 1.7);
    const half = box.h * HALF * (0.06 + 0.94 * open);
    const gap = box.h * GAP * open;
    const ws = BANDS.map((one) => one(u));
    const total = ws.reduce((a, b) => a + b, 0);
    const room = 2 * half - gap * (ws.length - 1);
    let y = cy - half;
    const out: [number, number][] = [];
    for (const one of ws) {
      out.push([y, y + (room * one) / total]);
      y += (room * one) / total + gap;
    }
    return { x: x0 + (box.x + box.w - x0) * u, out };
  };
  const cols = Array.from({ length: STEPS + 1 }, (_, i) => at(i / STEPS));
  const fills: Color[] = [paint.white, [255, 255, 255, 200], [255, 255, 255, 140]];
  BANDS.forEach((_, b) => {
    const top = cols.map((one): Point => [one.x, one.out[b][0]]);
    const bottom = cols.map((one): Point => [one.x, one.out[b][1]]).reverse();
    pen.polygon([...top, ...bottom], fills[b]);
  });
  pen.disc(x0, cy, box.w * SPRING, paint.white);
}
