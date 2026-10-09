import { frame, Grid } from "mrlyjs/view";
import type { Color, Ink, Pen } from "mrlyjs/view";

export const units = {};

const SIDE = 33;
const CELL = 25;
const CENTRE = [20, 20];
const BODY = 7.6;
const SUN = [-0.62, -0.58, 0.53];
const ARCS: [number, number, number][] = [
  [12, 3.3, 4.5],
  [15, 3.05, 4.65],
  [18, 3.2, 4.4],
];

const unit = (v: number[]) => {
  const l = Math.hypot(...v);
  return v.map((c) => c / l);
};

export default function draw(pen: Pen, ink: Ink) {
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, 0);
  const sun = unit(SUN);
  const rank = new Int8Array(SIDE * SIDE);
  const tone: Color[] = new Array(SIDE * SIDE);
  const seen = { lit: 0, dark: 0, heads: 0 };
  const put = (col: number, row: number, level: number, color: Color) => {
    if (col < 0 || row < 0 || col >= SIDE || row >= SIDE) return;
    const at = row * SIDE + col;
    if (level <= rank[at]) return;
    rank[at] = level;
    tone[at] = color;
  };
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const x = col - CENTRE[0];
      const y = row - CENTRE[1];
      if (Math.hypot(x, y) > BODY) continue;
      const z = Math.sqrt(Math.max(BODY * BODY - x * x - y * y, 0));
      const light = (x * sun[0] + y * sun[1] + z * sun[2]) / BODY;
      if (light > 0.42) seen.lit++;
      if (light <= -0.1) seen.dark++;
      put(col, row, 9, light > 0.42 ? ink.fg : light > 0.1 ? ink.blue : light > -0.1 ? ink.mix(ink.ground, ink.blue, 0.5) : ink.mix(ink.ground, ink.dim, 0.45));
    }
  }
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const x = col - CENTRE[0];
      const y = row - CENTRE[1];
      const a = Math.atan2(y, x) + (y < 0 ? 2 * Math.PI : 0);
      for (const [r, head, tail] of ARCS) {
        if (Math.abs(Math.hypot(x, y) - r) > 0.5 || a < head || a > tail) continue;
        if (Math.hypot(x, y) <= BODY + 1) throw new Error("app-voyage: a streak touches the planet");
        const t = (tail - a) / (tail - head);
        const level = t > 0.75 ? 3 : t > 0.35 ? 2 : 1;
        if (level === 3) seen.heads++;
        put(col, row, level, level === 3 ? ink.fg : level === 2 ? ink.cyan : ink.mix(ink.ground, ink.blue, 0.55));
      }
    }
  }
  if (!seen.lit || !seen.dark) throw new Error("app-voyage: the terminator does not cross the planet");
  if (!seen.heads) throw new Error("app-voyage: the streaks have no heads");
  for (let at = 0; at < SIDE * SIDE; at++) if (rank[at]) cells.fill(pen, at % SIDE, Math.floor(at / SIDE), tone[at]);
}
