import * as life from "mrlyjs/life";
import type { Ink, Pen } from "mrlyjs/view";

const SIDE = 96;
const STEPS = 512;
const SEED = [
  [1, 0],
  [2, 0],
  [0, 1],
  [1, 1],
  [1, 2],
];

export const units = { life };

// LIFE

let memo: { first: Int32Array; live: Uint8Array } | undefined;

function run() {
  if (memo) return memo;
  const mask = life.design_mask(2, 7, 3, 1);
  if (mask.shape.length !== 2 || mask.shape[0] !== 3 || mask.shape[1] !== 3) throw new Error(`demo-life: mask ${mask.shape}, want 3,3`);
  let sites = 0;
  for (const v of mask.data) if (v === 1) sites++;
  if (sites !== 8) throw new Error(`demo-life: mask holds ${sites} sites, want 8`);

  const types = new Uint8Array(SIDE * SIDE);
  const corner = SIDE / 2 - 1;
  for (const [x, y] of SEED) types[(corner + y) * SIDE + corner + x] = 1;
  const seeded = types.reduce((sum, v) => sum + (v === 1 ? 1 : 0), 0);
  if (seeded !== 5) throw new Error(`demo-life: ${seeded} seeded cells, want 5`);

  let cell: life.Cell = { shape: [SIDE, SIDE], types };
  const first = new Int32Array(SIDE * SIDE).fill(-1);
  for (let step = 0; step <= STEPS; step++) {
    const now = cell.types;
    for (let slot = 0; slot < first.length; slot++) if (first[slot] < 0 && now[slot] !== 0) first[slot] = step;
    if (step < STEPS) cell = life.next_grid(cell, [3], [2, 3], mask, "Constant");
  }
  const ever = first.reduce((sum, mark) => sum + (mark >= 0 ? 1 : 0), 0);
  const live = Uint8Array.from(cell.types, (v) => (v !== 0 ? 1 : 0));
  const alive = live.reduce((sum, bit) => sum + bit, 0);
  if (!(alive > 0)) throw new Error("demo-life: nothing alive at the end");
  if (!(ever > alive)) throw new Error(`demo-life: ${ever} cells ever alive, want over ${alive}`);
  memo = { first, live };
  return memo;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const { first, live } = run();
  const area = pen.frame(0.08);
  const scale = Math.max(Math.floor(area.w / SIDE), 1);
  const block = SIDE * scale;
  const ox = Math.round((pen.width - block) / 2);
  const oy = Math.round((pen.height - block) / 2);
  const ramp = ink.Ramp.tone(ink.green, ink.line);
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const slot = row * SIDE + col;
      const x = ox + col * scale;
      const y = oy + row * scale;
      if (first[slot] >= 0) pen.rect(x, y, scale, scale, ramp.at(first[slot] / STEPS));
      if (live[slot] !== 0) pen.rect(x, y, scale, scale, ink.yellow);
    }
  }
}
