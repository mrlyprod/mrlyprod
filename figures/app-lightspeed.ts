import { frame, Grid } from "mrlyjs/view";
import type { Color, Ink, Pen } from "mrlyjs/view";

export const units = {};

const SIDE = 33;
const CELL = 25;
const STARS = 24;
const GOLDEN = Math.PI * (3 - Math.sqrt(5));
const A = 0.7548776662;
const B = 0.569840291;

const frac = (v: number) => v - Math.floor(v);

export default function draw(pen: Pen, ink: Ink) {
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, 0);
  const mid = (SIDE - 1) / 2;
  const rank = new Int8Array(SIDE * SIDE);
  const tone: Color[] = new Array(SIDE * SIDE);
  const mark = (col: number, row: number, level: number, color: Color) => {
    if (col < 0 || row < 0 || col >= SIDE || row >= SIDE) return;
    const at = row * SIDE + col;
    if (level <= rank[at]) return;
    rank[at] = level;
    tone[at] = color;
  };
  for (let i = 0; i < STARS; i++) {
    const angle = i * GOLDEN;
    const head = (0.55 + 0.9 * frac(i * B)) * mid;
    const tail = (0.25 + 0.3 * frac(i * A)) * head;
    const hue = i % 3 === 0 ? ink.blue : ink.cyan;
    const dx = Math.cos(angle);
    const dy = Math.sin(angle);
    const steps = Math.ceil((head - tail) * Math.max(Math.abs(dx), Math.abs(dy)));
    for (let k = 0; k <= steps; k++) {
      const t = k / steps;
      const r = tail + (head - tail) * t;
      const level = t > 0.6 ? 3 : t > 0.25 ? 2 : 1;
      const color = level === 3 ? ink.fg : level === 2 ? hue : ink.mix(ink.ground, hue, 0.5);
      mark(Math.round(mid + dx * r), Math.round(mid + dy * r), level, color);
    }
  }
  if (rank[mid * SIDE + mid]) throw new Error("app-lightspeed: a streak crosses the vanishing point");
  for (let at = 0; at < SIDE * SIDE; at++) if (rank[at]) cells.fill(pen, at % SIDE, Math.floor(at / SIDE), tone[at]);
}
