import type { Cell } from "@mrlyprod/mrly/math";
import * as math from "@mrlyprod/mrly/math";
import type { Pen } from "#view";
import { Grid } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = { math };

const DIM = 2;
const BASE = 2;
const SIDE = 3;
const LEVEL = 2;
const COLS = 4;
const CLASSES = 6;
const CARPET = 7;
const SHOW = [1, 14, 6, 3, CARPET, 5, 13, 9, 11];
const ACROSS = 3;
const GAP = 0.05;

type Grown = { code: number; cell: Cell };

let rows: Grown[] | null = null;

function facts() {
  if (rows) return rows;
  const leaders = new Set(math.bang.universe_codes(DIM));
  const total = Number(math.bang.factory.total_codes(DIM, BASE));
  if (total !== COLS * COLS) throw new Error(`designs: ${total} codes do not fill a ${COLS} by ${COLS} board`);
  if (leaders.size !== CLASSES) throw new Error(`designs: ${leaders.size} classes, want ${CLASSES}`);
  if (SHOW.length !== ACROSS * ACROSS || SHOW[Math.floor(SHOW.length / 2)] !== CARPET)
    throw new Error(`designs: the ${ACROSS} by ${ACROSS} board must centre the carpet ${CARPET}`);
  if (!leaders.has(String(CARPET))) throw new Error(`designs: the carpet ${CARPET} does not lead its class`);
  rows = Array.from({ length: total }, (_, code) => ({ code, cell: math.two.create(code, SIDE, LEVEL, 0, BASE) }));
  const lit = rows.reduce((sum, row) => sum + math.two.fills(row.cell), 0);
  const want = rows.reduce((sum, row) => sum + Number(math.counts.fill(row.code, SIDE, DIM, LEVEL, BASE)), 0);
  if (lit !== want) throw new Error(`designs: the grown cells light ${lit}, the counts say ${want}`);
  return rows;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "indigo", 0.17);
  const all = facts();
  const panels = box.panels(ACROSS, ACROSS, box.w * GAP);
  SHOW.forEach((code, i) => {
    const { cell } = all[code];
    const [h, w] = cell.shape;
    const tone = code === CARPET ? paint.white : paint.light;
    new Grid(panels[i], w, h, 0).paint(pen, cell, (type) => (type ? tone : null));
  });
}
