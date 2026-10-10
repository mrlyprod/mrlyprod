import * as math from "mrlyjs/math";
import { Grid, frame } from "mrlyjs/view";
import type { Color, Frame, Ink, Pen } from "mrlyjs/view";

export const units = { math };

const NUMBER = 3;
const BASE = 3;
const LEVEL = 3;
const SIDE = 27;
const CODES = ["127", "239"];
const SEEDS = [7, 784];
const WALKERS = 90;
const STEPS = 160;
const MARGIN = 0.06;
const GAP = 0.06;
const FADE = 0.35;
const WARM = 0.45;
const WANT = { fills: 343 };
const MOVES = [[1, 0], [-1, 0], [0, 1], [0, -1]];

type Swarm = { filled: Uint8Array; home: number; at: number[]; seen: Uint8Array; spread: number };

let swarms: Swarm[] | null = null;

function walk(code: string, seed: number): Swarm {
  const cell = math.two.create(code, NUMBER, LEVEL, 0, BASE);
  if (cell.shape[0] !== SIDE || cell.shape[1] !== SIDE) throw new Error(`app-race: design ${code} is ${cell.shape}, want ${SIDE} square`);
  const filled = Uint8Array.from(cell.types, (type) => (type ? 1 : 0));
  const fills = filled.reduce((sum, on) => sum + on, 0);
  if (fills !== WANT.fills) throw new Error(`app-race: design ${code} fills ${fills}, want ${WANT.fills}`);
  const centre = (SIDE - 1) / 2;
  let home = -1;
  let least = Infinity;
  for (let i = 0; i < filled.length; i++) {
    if (!filled[i]) continue;
    const d = (Math.floor(i / SIDE) - centre) ** 2 + ((i % SIDE) - centre) ** 2;
    if (d < least) {
      least = d;
      home = i;
    }
  }
  const rng = new math.Rng(seed);
  const rand = () => rng.unit();
  const at = Array.from({ length: WALKERS }, () => home);
  const seen = new Uint8Array(filled.length);
  seen[home] = 1;
  for (let step = 0; step < STEPS; step++) {
    for (let w = 0; w < at.length; w++) {
      const r = Math.floor(at[w] / SIDE);
      const c = at[w] % SIDE;
      const [dr, dc] = MOVES[Math.floor(rand() * 4)];
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nc >= 0 && nr < SIDE && nc < SIDE && filled[nr * SIDE + nc]) at[w] = nr * SIDE + nc;
      seen[at[w]] = 1;
    }
  }
  rng.free();
  if (at.some((i) => !filled[i])) throw new Error(`app-race: a walker of design ${code} is off the filled sites`);
  const hr = Math.floor(home / SIDE);
  const hc = home % SIDE;
  const spread = Math.sqrt(at.reduce((sum, i) => sum + (Math.floor(i / SIDE) - hr) ** 2 + ((i % SIDE) - hc) ** 2, 0) / at.length);
  return { filled, home, at, seen, spread };
}

function study(): Swarm[] {
  if (swarms) return swarms;
  const made = CODES.map((code, i) => walk(code, SEEDS[i]));
  if (made[0].spread <= made[1].spread) throw new Error(`app-race: ${CODES[0]} spreads ${made[0].spread.toFixed(2)}, ${CODES[1]} ${made[1].spread.toFixed(2)}, the fast one should lead`);
  swarms = made;
  return swarms;
}

function team(pen: Pen, ink: Ink, box: Frame, swarm: Swarm, tint: Color) {
  const cells = new Grid(box, SIDE, SIDE, 0);
  const faint = ink.mix(ink.ground, tint, FADE);
  const warm = ink.mix(ink.ground, tint, WARM + FADE);
  cells.paint(pen, { shape: [SIDE, SIDE], types: swarm.filled }, (type) => (type ? faint : null));
  cells.paint(pen, { shape: [SIDE, SIDE], types: swarm.seen }, (type) => (type ? warm : null));
  const unit = box.w / SIDE;
  const spot = (i: number): [number, number] => [box.x + ((i % SIDE) + 0.5) * unit, box.y + (Math.floor(i / SIDE) + 0.5) * unit];
  for (const walker of swarm.at) {
    const [x, y] = spot(walker);
    pen.disc(x, y, unit * 0.5, tint);
  }
  const [hx, hy] = spot(swarm.home);
  pen.ring(hx, hy, swarm.spread * unit, unit * 0.35, tint);
  pen.rect(hx - unit * 0.5, hy - unit * 0.5, unit, unit, ink.fg);
}

export default function draw(pen: Pen, ink: Ink) {
  const [fast, slow] = study();
  const area = pen.area(MARGIN);
  const gap = area.w * GAP;
  const side = Math.min((area.w - gap) / 2, area.h);
  const top = area.y + (area.h - side) / 2;
  const left = area.x + (area.w - 2 * side - gap) / 2;
  team(pen, ink, frame(left, top, side, side), fast, ink.blue);
  team(pen, ink, frame(left + side + gap, top, side, side), slow, ink.orange);
}
