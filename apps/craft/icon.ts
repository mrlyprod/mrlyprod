import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

export const units = {};

const STATIONS = 6;
const RING = 0.36;
const SHOP = 0.1;
const ROUND = 0.03;
const PLAZA = 0.14;
const BOT = 0.034;
const NEAR = 0.215;
const SPREAD = 0.22;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "orange", 0.16);
  const [cx, cy] = box.center();
  const s = box.w;
  pen.disc(cx, cy, s * PLAZA, paint.light);
  for (let i = 0; i < STATIONS; i++) {
    const a = (i / STATIONS) * 2 * Math.PI - Math.PI / 2;
    const x = cx + Math.cos(a) * s * RING;
    const y = cy + Math.sin(a) * s * RING;
    pen.round_rect(x - s * SHOP, y - s * SHOP, s * SHOP * 2, s * SHOP * 2, s * ROUND, paint.white);
    for (const k of [-1, 1]) {
      const b = a + k * SPREAD;
      pen.disc(cx + Math.cos(b) * s * NEAR, cy + Math.sin(b) * s * NEAR, s * BOT, paint.white);
    }
  }
}
