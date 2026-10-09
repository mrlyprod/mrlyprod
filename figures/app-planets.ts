import { frame, Grid } from "mrlyjs/view";
import type { Color, Ink, Pen } from "mrlyjs/view";

export const units = {};

const SIDE = 33;
const CELL = 25;
const BODY = 8.6;
const TILT = -0.32;
const SQUASH = 0.27;
const RING: [number, number] = [11, 15.6];
const GAP: [number, number] = [13.4, 14.1];
const SUN = [-0.72, -0.42, 0.55];

const unit = (v: number[]) => {
  const l = Math.hypot(...v);
  return v.map((c) => c / l);
};

export default function draw(pen: Pen, ink: Ink) {
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, 0);
  const mid = (SIDE - 1) / 2;
  const sun = unit(SUN);
  const at = (col: number, row: number) => {
    const x = col - mid;
    const y = row - mid;
    const u = x * Math.cos(TILT) + y * Math.sin(TILT);
    const v = (-x * Math.sin(TILT) + y * Math.cos(TILT)) / SQUASH;
    const r = Math.hypot(u, v);
    const disc = Math.hypot(x, y) <= BODY;
    const ring = r >= RING[0] && r <= RING[1] && !(r > GAP[0] && r < GAP[1]);
    return { x, y, r, disc, ring, front: ring && v > 0 };
  };
  const seen = { lit: 0, dark: 0, front: 0, back: 0, shade: 0 };
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const { x, y, r, disc, ring, front } = at(col, row);
      let tone: Color | null = null;
      if (front || (ring && !disc)) {
        tone = r < GAP[0] ? ink.cyan : ink.mix(ink.ground, ink.cyan, 0.55);
        if (front && disc) seen.front++;
        if (!front) seen.back++;
      } else if (disc && row + 1 < SIDE && at(col, row + 1).front) {
        seen.shade++;
      } else if (disc) {
        const z = Math.sqrt(Math.max(BODY * BODY - x * x - y * y, 0));
        const light = (x * sun[0] + y * sun[1] + z * sun[2]) / BODY;
        tone = light > 0.42 ? ink.fg : light > 0.1 ? ink.blue : light > -0.1 ? ink.mix(ink.ground, ink.blue, 0.5) : ink.mix(ink.ground, ink.dim, 0.45);
        if (light > 0.42) seen.lit++;
        if (light <= -0.1) seen.dark++;
      }
      if (tone) cells.fill(pen, col, row, tone);
    }
  }
  if (!seen.lit || !seen.dark) throw new Error("app-planets: the terminator does not cross the disc");
  if (!seen.front || !seen.back) throw new Error("app-planets: the rings do not pass both in front of and behind the disc");
  if (!seen.shade) throw new Error("app-planets: the rings cast no shadow on the disc");
}
