import * as math from "mrlyjs/math";
import { Grid, frame, type Color, type Frame, type Ink, type Pen } from "mrlyjs/view";
import census from "./census/demo-race.json" with { type: "json" };

const NUMBER = 3;
const BASE = 3;
const LEVEL = 4;
const SIDE = 81;
const WALKERS = 300;

export const units = { math };

type Swarm = { home: number; spread: number; at: number[] };

// SWARM

function filled(code: string, swarm: Swarm) {
  const cell = math.two.create(code, NUMBER, LEVEL, 0, BASE);
  if (cell.shape[0] !== SIDE || cell.shape[1] !== SIDE || cell.types.length !== SIDE * SIDE) throw new Error(`demo-race: design ${code} is ${cell.shape}, want ${SIDE} square`);
  const on = Array.from(cell.types, (kind) => kind !== 0);
  if (on.filter(Boolean).length !== 2401) throw new Error(`demo-race: design ${code} fills ${on.filter(Boolean).length}, want 2401`);
  if (swarm.at.length !== WALKERS) throw new Error(`demo-race: ${swarm.at.length} walkers, want ${WALKERS}`);
  if (swarm.at.some((flat) => !Number.isInteger(flat) || flat < 0 || flat >= SIDE * SIDE || !on[flat])) throw new Error(`demo-race: a walker of design ${code} is off the filled sites`);
  if (!Number.isInteger(swarm.home) || swarm.home < 0 || swarm.home >= SIDE * SIDE || !on[swarm.home]) throw new Error(`demo-race: home of design ${code} is off the filled sites`);
  return on;
}

function team(pen: Pen, ink: Ink, box: Frame, on: boolean[], swarm: Swarm, tint: Color) {
  const grid = new Grid(box, SIDE, SIDE, 0);
  for (let flat = 0; flat < SIDE * SIDE; flat++) {
    if (on[flat]) grid.fill(pen, flat % SIDE, Math.floor(flat / SIDE), ink.mix(ink.line, ink.dim, 0.35));
  }
  const unit = box.w / SIDE;
  const spot = (flat: number): [number, number] => [box.x + (flat % SIDE) * unit + unit / 2, box.y + Math.floor(flat / SIDE) * unit + unit / 2];
  const [hx, hy] = spot(swarm.home);
  pen.ring(hx, hy, swarm.spread * unit, unit * 0.5, tint);
  for (const walker of swarm.at) {
    const [x, y] = spot(walker);
    pen.disc(x, y, unit * 0.62, tint);
  }
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const { fast, slow } = census as { fast: Swarm; slow: Swarm };
  const onFast = filled("127", fast);
  const onSlow = filled("239", slow);
  const side = area.w * 0.55;
  team(pen, ink, frame(area.x, area.y, side, side), onFast, fast, ink.blue);
  team(pen, ink, frame(area.x + area.w - side, area.y + area.h - side, side, side), onSlow, slow, ink.orange);
}
