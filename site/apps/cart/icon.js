import { plate } from "../../lib/icon.js";

export const units = {};

const STROKE = 0.075;
const HANDLE = [[0.03, 0.1], [0.19, 0.1], [0.33, 0.64]];
const BASKET = [[0.24, 0.3], [0.97, 0.3], [0.86, 0.64], [0.33, 0.64]];
const WHEELS = [[0.43, 0.86], [0.78, 0.86]];
const WHEEL = 0.1;
const HUB = 0.028;

export default function draw(pen) {
  const { box, paint } = plate(pen, "orange", 0.18);
  const at = ([u, v]) => box.at(u, v);
  const thick = box.w * STROKE;
  pen.polyline(HANDLE.map(at), thick, paint.white);
  pen.polygon(BASKET.map(at), paint.white);
  pen.polyline([...BASKET, BASKET[0]].map(at), thick, paint.white);
  for (const u of [0.46, 0.64]) pen.segment(at([u, 0.3]), at([u, 0.64]), thick * 0.4, paint.light);
  for (const wheel of WHEELS) {
    pen.disc(...at(wheel), box.w * WHEEL, paint.white);
    pen.disc(...at(wheel), box.w * (WHEEL - 0.018), paint.deep);
    pen.disc(...at(wheel), box.w * HUB, paint.white);
  }
}
