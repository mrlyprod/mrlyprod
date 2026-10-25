import * as math from "@mrlyprod/mrly/math";
import type { Color, Pen } from "#view";
import { Grid } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = { math };

const NUMBER = 3;
const BASE = 3;
const LEVEL = 3;
const SIDE = 27;
const CODES = ["127", "239"];
const SEEDS = [7, 784];
const WALKERS = 90;
const STEPS = 120;
const TRAIL = 130;
const RING = 0.95;
const WANT = { fills: 343 };
const MOVES = [
  [1, 0],
  [-1, 0],
  [0, 1],
  [0, -1],
];

type Swarm = { filled: Uint8Array; home: number; at: number[]; seen: Uint8Array; spread: number };

let swarms: Swarm[] | null = null;

function walk(code: string, seed: number): Swarm {
  const cell = math.two.create(code, NUMBER, LEVEL, 0, BASE);
  if (cell.shape[0] !== SIDE || cell.shape[1] !== SIDE)
    throw new Error(`race: design ${code} is ${cell.shape}, want ${SIDE} square`);
  const filled = Uint8Array.from(cell.types, (type) => (type ? 1 : 0));
  const fills = filled.reduce((sum, on) => sum + on, 0);
  if (fills !== WANT.fills) throw new Error(`race: design ${code} fills ${fills}, want ${WANT.fills}`);
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
  if (at.some((i) => !filled[i])) throw new Error(`race: a walker of design ${code} is off the filled sites`);
  const hr = Math.floor(home / SIDE);
  const hc = home % SIDE;
  const spread = Math.sqrt(
    at.reduce((sum, i) => sum + (Math.floor(i / SIDE) - hr) ** 2 + ((i % SIDE) - hc) ** 2, 0) / at.length,
  );
  return { filled, home, at, seen, spread };
}

function study() {
  if (swarms) return swarms;
  const made = CODES.map((code, i) => walk(code, SEEDS[i]));
  if (made[0].spread <= made[1].spread)
    throw new Error(
      `race: ${CODES[0]} spreads ${made[0].spread.toFixed(2)}, ${CODES[1]} ${made[1].spread.toFixed(2)}, the fast one should lead`,
    );
  swarms = made;
  return swarms;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "red", 0.16);
  const [fast, slow] = study();
  const cells = new Grid(box, SIDE, SIDE, 0);
  const shape = [SIDE, SIDE];
  const unit = box.w / SIDE;
  const spot = (i: number) => [box.x + ((i % SIDE) + 0.5) * unit, box.y + (Math.floor(i / SIDE) + 0.5) * unit];
  const [hx, hy] = spot(fast.home);
  const finish = box.radius() * RING;
  const inside = (i: number) => {
    const [x, y] = spot(i);
    return Math.hypot(x - hx, y - hy) < finish - unit;
  };
  const clip = (seen: Uint8Array) => Uint8Array.from(seen, (on, i) => (on && inside(i) ? 1 : 0));
  cells.paint(pen, { shape, types: clip(slow.seen) }, (type) =>
    type ? ([...paint.light.slice(0, 3), TRAIL] as Color) : null,
  );
  cells.paint(pen, { shape, types: clip(fast.seen) }, (type) => (type ? [255, 255, 255, TRAIL] : null));
  for (const walker of slow.at) {
    if (!inside(walker)) continue;
    const [x, y] = spot(walker);
    pen.disc(x, y, unit * 0.55, paint.light);
  }
  for (const walker of fast.at) {
    if (!inside(walker)) continue;
    const [x, y] = spot(walker);
    pen.disc(x, y, unit * 0.55, paint.white);
  }
  pen.ring(hx, hy, finish, unit * 1.5, paint.white);
  pen.rect(hx - unit * 0.7, hy - unit * 0.7, unit * 1.4, unit * 1.4, paint.ink);
}
