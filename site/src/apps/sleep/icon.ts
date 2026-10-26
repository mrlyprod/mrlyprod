import * as math from "@mrlyprod/mrly/math";
import type { Cell, Color, Pen } from "#view";
import { Grid, grid } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = { math };

const TILE = 5;
const SIDE = 11;
const STEP = 3;
const ALPHA = [255, 140, 60];
const LIT = 21;

let memo: Cell | null = null;

function study(): Cell {
  if (memo) return memo;
  const tile = math.atoms.carpet_2d(TILE);
  if ([...tile.data].join("") !== grid.LOGO.join("")) throw new Error(`sleep: the carpet of ${TILE} is not the mark`);
  const spots = ALPHA.map((_, k) => [k * STEP, SIDE - TILE - k * STEP]);
  const far = spots[spots.length - 1];
  if (far[0] + TILE !== SIDE || far[1] !== 0)
    throw new Error(`sleep: the trail ends at ${far}, not in the far corner of ${SIDE}`);
  const owner = new Int8Array(SIDE * SIDE).fill(-1);
  const types = new Uint8Array(SIDE * SIDE);
  spots.forEach(([top, left], k) => {
    for (let row = 0; row < TILE; row++) {
      for (let col = 0; col < TILE; col++) {
        const i = (top + row) * SIDE + left + col;
        if (owner[i] >= 0) continue;
        owner[i] = k;
        if (tile.data[row * TILE + col]) types[i] = k + 1;
      }
    }
  });
  const shown = ALPHA.map((_, k) => types.filter((t) => t === k + 1).length);
  if (shown[0] !== LIT || shown.some((n) => n === 0))
    throw new Error(`sleep: the layers show ${shown} cells, want ${LIT} for the tile and some for every ghost`);
  memo = { shape: [SIDE, SIDE], types };
  return memo;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "indigo", 0.16);
  const [r, g, b] = paint.white;
  const tones = [null, ...ALPHA.map((a): Color => [r, g, b, a])];
  new Grid(box, SIDE, SIDE, 0).paint(pen, study(), (type) => tones[type]);
}
