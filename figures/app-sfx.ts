import { frame, Grid } from "mrlyjs/view";
import type { Color, Ink, Pen } from "mrlyjs/view";

export const units = {};

const SIDE = 33;
const CELL = 25;
const BARS = 16;
const PEAK = 2;
const DECAY = 4.5;
const REACH = 15;

const envelope = (k: number) => (k < PEAK ? (k + 1) / (PEAK + 1) : Math.exp(-(k - PEAK) / DECAY));

export default function draw(pen: Pen, ink: Ink) {
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, 0);
  const mid = (SIDE - 1) / 2;
  const heights = Array.from({ length: BARS }, (_, k) => Math.max(0, Math.round(REACH * envelope(k) * (0.62 + 0.38 * Math.abs(Math.cos(k * 1.9))))));
  const top = heights.indexOf(Math.max(...heights));
  if (top > BARS / 4) throw new Error(`app-sfx: the burst peaks at bar ${top}, not in its first quarter`);
  if (heights.at(-1)! > 1) throw new Error("app-sfx: the burst does not die away");
  for (let col = 0; col < SIDE; col++) cells.fill(pen, col, mid, ink.dim);
  heights.forEach((h, k) => {
    const env = envelope(k);
    const tone: Color = env > 0.6 ? ink.fg : env > 0.25 ? ink.blue : ink.mix(ink.ground, ink.blue, 0.55);
    const col = 1 + 2 * k;
    for (let row = mid - h; row <= mid + h; row++) cells.fill(pen, col, row, tone);
  });
}
