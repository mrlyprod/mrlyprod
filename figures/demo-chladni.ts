import * as life from "mrlyjs/life";
import type { Ink, Pen } from "mrlyjs/view";
import census from "./census/demo-chladni.json" with { type: "json" };

const SIZE = 256;
const CELL = 3;
const STAMP = 6;
const CODE = 7;
const BASE = 3;
const LEVEL = 3;
const SPAN = 27;
const BUDGET = 512;

export const units = { life };

// FIELD

function field(rows: string[]) {
  if (rows.length !== SIZE) throw new Error(`demo-chladni: ${rows.length} rows, want ${SIZE}`);
  const types = new Uint8Array(SIZE * SIZE);
  rows.forEach((row, r) => {
    if (row.length !== SIZE / 4 || !/^[0-9a-f]+$/.test(row)) throw new Error(`demo-chladni: row ${r} is not ${SIZE / 4} hex digits`);
    for (let c = 0; c < SIZE; c++) types[r * SIZE + c] = (parseInt(row[c >> 2], 16) >> (3 - (c & 3))) & 1;
  });
  return types;
}

function stamp() {
  const mask = life.design_mask(2, CODE, BASE, LEVEL);
  if (mask.shape.length !== 2 || mask.shape[0] !== SPAN || mask.shape[1] !== SPAN) throw new Error(`demo-chladni: mask ${mask.shape}, want ${SPAN},${SPAN}`);
  if (BASE ** LEVEL !== SPAN) throw new Error(`demo-chladni: ${BASE} to the ${LEVEL} is not ${SPAN}`);
  let sum = 0;
  for (const v of mask.data) sum += v;
  if (sum !== BUDGET) throw new Error(`demo-chladni: mask sums to ${sum}, want ${BUDGET}`);
  if (mask.data[(SPAN >> 1) * SPAN + (SPAN >> 1)] !== 0) throw new Error("demo-chladni: mask centre is not empty");
  return mask.data;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const types = field((census as { rows: string[] }).rows);
  const mask = stamp();
  const area = pen.frame(0.04);
  let live = 0;
  for (const t of types) live += t;
  if (!(live > 0)) throw new Error("demo-chladni: nothing alive");

  const block = SIZE * CELL;
  const ox = Math.round(area.x + area.w - block);
  const oy = Math.round(area.y + area.h - block);
  for (let row = 0; row < SIZE; row++) {
    for (let col = 0; col < SIZE; col++) {
      if (types[row * SIZE + col] !== 0) pen.rect(ox + col * CELL, oy + row * CELL, CELL, CELL, ink.blue);
    }
  }

  const sx = Math.round(area.x);
  const sy = Math.round(area.y);
  let stamped = 0;
  for (let row = 0; row < SPAN; row++) {
    for (let col = 0; col < SPAN; col++) {
      if (mask[row * SPAN + col] === 1) {
        pen.rect(sx + col * STAMP, sy + row * STAMP, STAMP, STAMP, ink.yellow);
        stamped++;
      }
    }
  }
  if (stamped !== BUDGET) throw new Error(`demo-chladni: ${stamped} cells stamped, want ${BUDGET}`);
}
