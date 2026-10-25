import type { Ink, Pen } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = {};

const BARS = 16;
const PEAK = 2;
const DECAY = 4.5;
const REACH = 15;
const WIDTH = 0.74;

const envelope = (k: number) => (k < PEAK ? (k + 1) / (PEAK + 1) : Math.exp(-(k - PEAK) / DECAY));

export default function draw(pen: Pen, ink: Ink) {
  const { box, paint } = plate(pen, "pink", 0.16);
  const heights = Array.from({ length: BARS }, (_, k) =>
    Math.max(0, Math.round(REACH * envelope(k) * (0.62 + 0.38 * Math.abs(Math.cos(k * 1.9))))),
  );
  const top = heights.indexOf(Math.max(...heights));
  if (top > BARS / 4) throw new Error(`sfx: the burst peaks at bar ${top}, not in its first quarter`);
  if (heights.at(-1)! > 1) throw new Error("sfx: the burst does not die away");
  const [, cy] = box.center();
  const slot = box.w / BARS;
  const w = slot * WIDTH;
  heights.forEach((h, k) => {
    const half = Math.max(w / 2, (h / REACH) * (box.h / 2));
    const x = box.x + k * slot + (slot - w) / 2;
    pen.round_rect(x, cy - half, w, 2 * half, w / 2, ink.mix(paint.light, paint.white, Math.min(1, envelope(k) / 0.6)));
  });
}
