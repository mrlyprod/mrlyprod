import * as math from "mrlyjs/math";
import { Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const DIM = 2;
const BASE = 2;
const SIDE = 3;
const LEVEL = 2;
const COLS = 4;
const MARGIN = 0.08;
const GAP = 14;
const CLASSES = 6;

type Cell = { shape: number[]; types: ArrayLike<number> };
type Row = { code: number; cell: Cell; leads: boolean };

let rows: Row[] | null = null;

function facts() {
  if (rows) return rows;
  const leaders = new Set(math.bang.universe_codes(DIM));
  const total = Number(math.bang.factory.total_codes(DIM, BASE));
  if (total !== COLS * COLS) throw new Error(`app-designs: ${total} codes do not fill a ${COLS} by ${COLS} board`);
  if (leaders.size !== CLASSES) throw new Error(`app-designs: ${leaders.size} classes, want ${CLASSES}`);
  rows = Array.from({ length: total }, (_, code) => ({ code, cell: math.two.create(code, SIDE, LEVEL, 0, BASE), leads: leaders.has(String(code)) }));
  const lit = rows.reduce((sum, row) => sum + math.two.fills(row.cell), 0);
  const want = rows.reduce((sum, row) => sum + Number(math.counts.fill(row.code, SIDE, DIM, LEVEL, BASE)), 0);
  if (lit !== want) throw new Error(`app-designs: the grown cells light ${lit}, the counts say ${want}`);
  return rows;
}

export default function draw(pen: Pen, ink: Ink) {
  const faded = ink.mix(ink.ground, ink.blue, 0.45);
  const panels = pen.frame(MARGIN).panels(COLS, COLS, GAP);
  facts().forEach(({ cell, leads }, i) => {
    const [h, w] = cell.shape;
    new Grid(panels[i], w, h, 0).paint(pen, cell, (type) => (type ? (leads ? ink.blue : faded) : null));
  });
}
