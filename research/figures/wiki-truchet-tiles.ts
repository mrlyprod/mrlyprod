import * as core from "mrlyjs/core";
import { field, type Ink, type Pen, type Point } from "mrlyjs/view";
import census from "./census/wiki-truchet-tiles.json" with { type: "json" };

const SIDE = 16;
const SEED = 7;
const SAMPLES = 3;
const MARGIN = 0.08;
const CROSS = [[1, 0], [0, 1]];
const DIAGONAL = [[0, 0], [1, 1]];

export const units = { core };

// FIELD

type Memo = { turns: boolean[]; hits: Uint8Array; x0: number; span: number };

function even(turns: boolean[], u: number, v: number) {
  const col = Math.min(Math.floor(u), SIDE - 1);
  const row = Math.min(Math.floor(v), SIDE - 1);
  const turn = turns[row * SIDE + col];
  const x = u - col;
  const y = v - row;
  const corner = (c: number[]) => (row + col + c[0] + c[1]) % 2 === 0;
  const centres = turn ? CROSS : DIAGONAL;
  for (const c of centres) {
    if (Math.hypot(x - c[0], y - c[1]) < 0.5) return corner(c);
  }
  return !corner(centres[0]);
}

let memo: Memo | undefined;

function facts(pen: Pen): Memo {
  if (memo) return memo;
  if (census.open.length !== 2 * SIDE * SIDE) throw new Error(`wiki-truchet-tiles: ${census.open.length} arcs, want ${2 * SIDE * SIDE}`);
  if (census.open.some((o) => o !== 0 && o !== 1)) throw new Error("wiki-truchet-tiles: an arc flag is not 0 or 1");
  const rng = new core.Rng(SEED);
  const turns = Array.from({ length: SIDE * SIDE }, () => rng.boolean());
  const frame = pen.frame(MARGIN);
  const cell = frame.w / SIDE;
  const x0 = Math.floor(frame.x);
  const x1 = Math.ceil(frame.x + frame.w);
  const span = x1 - x0;
  const hits = new Uint8Array(span * span);
  for (let py = x0; py < x1; py++) {
    for (let px = x0; px < x1; px++) {
      let count = 0;
      for (let sy = 0; sy < SAMPLES; sy++) {
        for (let sx = 0; sx < SAMPLES; sx++) {
          const x = px + (sx + 0.5) / SAMPLES;
          const y = py + (sy + 0.5) / SAMPLES;
          const u = (x - frame.x) / cell;
          const v = (y - frame.y) / cell;
          const inside = u >= 0 && u < SIDE && v >= 0 && v < SIDE;
          if (inside && even(turns, u, v)) count++;
        }
      }
      hits[(py - x0) * span + (px - x0)] = count;
    }
  }
  memo = { turns, hits, x0, span };
  return memo;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const { turns, hits, x0, span } = facts(pen);
  const frame = pen.frame(MARGIN);
  const cell = frame.w / SIDE;
  const tint = ink.fade(ink.dim, 0.25);
  const tiles = field.patch(x0, x0, span, span);
  for (let py = x0; py < x0 + span; py++) {
    for (let px = x0; px < x0 + span; px++) tiles.blend(px, py, tint, hits[(py - x0) * span + (px - x0)] / (SAMPLES * SAMPLES));
  }
  tiles.paint(pen);

  const thick = cell * 0.15;
  const radius = cell / 2;
  let arc = 0;
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const x = frame.x + col * cell;
      const y = frame.y + row * cell;
      const sweeps: [Point, [number, number]][] = turns[row * SIDE + col]
        ? [
            [[x + cell, y], [Math.PI / 2, Math.PI]],
            [[x, y + cell], [-Math.PI / 2, 0]],
          ]
        : [
            [[x, y], [0, Math.PI / 2]],
            [[x + cell, y + cell], [Math.PI, Math.PI + Math.PI / 2]],
          ];
      for (const [centre, angles] of sweeps) {
        pen.arc(centre, radius, angles, thick, census.open[arc] ? ink.blue : ink.yellow);
        arc++;
      }
    }
  }
  const w = pen.width;
  const end = frame.x + frame.w;
  pen.rect(0, 0, w, frame.y, ink.ground);
  pen.rect(0, end, w, w - end, ink.ground);
  pen.rect(0, 0, frame.x, w, ink.ground);
  pen.rect(end, 0, w - end, w, ink.ground);
}
