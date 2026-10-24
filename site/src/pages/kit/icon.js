import { plate } from "../../lib/plate.js";

export const units = {};

const TRACKS = [
  [0.2, 0.68],
  [0.5, 0.3],
  [0.8, 0.78],
];
const TRACK = 0.075;
const KNOB = 0.125;
const HUB = 0.055;

export default function draw(pen) {
  const { box, paint } = plate(pen, "cyan", 0.16);
  for (const [v, u] of TRACKS)
    if (v <= 0 || v >= 1 || u <= 0 || u >= 1) throw new Error(`kit: knob ${u} on track ${v} leaves the frame`);
  const thick = box.w * TRACK;
  for (const [v, u] of TRACKS) {
    pen.segment(box.at(0, v), box.at(1, v), thick, paint.light);
    pen.segment(box.at(0, v), box.at(u, v), thick, paint.white);
    pen.disc(...box.at(u, v), box.w * KNOB, paint.white);
    pen.disc(...box.at(u, v), box.w * HUB, paint.deep);
  }
}
