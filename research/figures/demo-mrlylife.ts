import * as life from "mrlyjs/life";
import type { Ink, Pen } from "mrlyjs/view";

const SIDE = 192;
const GENERATIONS = 64;
const SEED = 1729;
const DENSITY = 0.05;
const CELL = 4;
const STAMP = 8;
const MASK = 9;
const SITES = 64;

export const units = { life };

// LIFE

let memo: { mask: Uint8Array; cells: Uint8Array } | undefined;

function soup(): life.Cell {
  const rng = new life.Rng(SEED);
  const types = new Uint8Array(SIDE * SIDE);
  for (let flat = 0; flat < types.length; flat++) types[flat] = rng.chance(DENSITY) ? 1 : 0;
  rng.free();
  return { shape: [SIDE, SIDE], types };
}

function run() {
  if (memo) return memo;
  const grid = life.design_mask(2, 7, 3, 2);
  if (grid.shape.length !== 2 || grid.shape[0] !== MASK || grid.shape[1] !== MASK) throw new Error(`demo-mrlylife: mask ${grid.shape}, want ${MASK},${MASK}`);
  const mask = Uint8Array.from(grid.data, (v) => (v === 1 ? 1 : 0));
  const sites = mask.reduce((sum, v) => sum + v, 0);
  if (sites !== SITES) throw new Error(`demo-mrlylife: mask holds ${sites} sites, want ${SITES}`);
  if (grid.data[Math.floor(MASK / 2) * MASK + Math.floor(MASK / 2)] !== 0) throw new Error("demo-mrlylife: the mask centre is not empty");
  if (life.lattice_index(grid) !== 1) throw new Error(`demo-mrlylife: lattice index ${life.lattice_index(grid)}, want 1`);

  let cell = soup();
  for (let generation = 0; generation < GENERATIONS; generation++) cell = life.next_grid(cell, [3], [2, 3], grid, "Wrap");
  if (cell.shape[0] !== SIDE || cell.shape[1] !== SIDE) throw new Error(`demo-mrlylife: grid ${cell.shape}, want ${SIDE},${SIDE}`);
  const cells = Uint8Array.from(cell.types, (v) => (v !== 0 ? 1 : 0));
  if (!cells.some((bit) => bit !== 0)) throw new Error("demo-mrlylife: nothing alive at the end");
  memo = { mask, cells };
  return memo;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const { mask, cells } = run();
  const area = pen.frame(0.08);
  const block = SIDE * CELL;
  const ox = Math.round(area.x + area.w - block);
  const oy = Math.round(area.y + area.h - block);
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      if (cells[row * SIDE + col] !== 0) pen.rect(ox + col * CELL, oy + row * CELL, CELL, CELL, ink.blue);
    }
  }

  const sx = Math.round(area.x);
  const sy = Math.round(area.y);
  let stamped = 0;
  for (let row = 0; row < MASK; row++) {
    for (let col = 0; col < MASK; col++) {
      if (mask[row * MASK + col] === 1) {
        pen.rect(sx + col * STAMP, sy + row * STAMP, STAMP, STAMP, ink.yellow);
        stamped++;
      }
    }
  }
  if (stamped !== SITES) throw new Error(`demo-mrlylife: ${stamped} stamped, want ${SITES}`);
}
