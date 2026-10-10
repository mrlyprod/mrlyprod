import type { Ink, Pen } from "mrlyjs/view";

const P = 7;
const LEVELS = 3;
const GAP = 0.03;
const UNIT = 3;
const SPACES = [8, 4, 1];

function kept(state: number, digit: number): number | null {
  if (state === 0) {
    if (digit <= Math.floor((P - 1) / 3)) return 0;
    if (digit >= (P + 1) / 2 && digit <= Math.floor((2 * P - 1) / 3)) return 1;
    return null;
  }
  if (digit <= Math.floor((P - 2) / 3)) return 0;
  if (digit >= (P - 1) / 2 && digit <= Math.floor((2 * P - 2) / 3)) return 1;
  return null;
}

function rows(): (number | null)[][] {
  const out: (number | null)[][] = [];
  let row: (number | null)[] = [0];
  for (let r = 0; r < LEVELS; r++) {
    row = row.flatMap((state) => Array.from({ length: P }, (_, d) => (state === null ? null : kept(state, d))));
    out.push(row);
  }
  return out;
}

function member(k: number): boolean {
  let state = 0;
  for (let rest = k; rest > 0; rest = Math.floor(rest / P)) {
    const next = kept(state, rest % P);
    if (next === null) return false;
    state = next;
  }
  return true;
}

export default function draw(pen: Pen, ink: Ink) {
  const board = rows();
  board.forEach((row, r) => {
    const fill = 4 ** (r + 1);
    const ends = [0, 1].map((s) => row.filter((c) => c === s).length);
    if (ends[0] !== (2 * fill + 1) / 3 || ends[1] !== (fill - 1) / 3) throw new Error(`demo-kummer: level ${r + 1} ends ${ends}, want ${(2 * fill + 1) / 3},${(fill - 1) / 3}`);
  });
  if (!member(4) || !member(14) || member(18)) throw new Error("demo-kummer: the witness 4, 14 in and 18 out fails");
  const area = pen.frame(0.08);
  const reach = Math.max(...board.map((row, r) => (row.findLastIndex((c) => c !== null) + 1) * P ** (LEVELS - 1 - r)));
  const x = Math.round(area.x + (area.w - reach * UNIT) / 2);
  const gap = Math.round(area.h * GAP);
  const tall = Math.floor((area.h - (LEVELS - 1) * gap) / LEVELS);
  const top = Math.round(area.y + (area.h - LEVELS * tall - (LEVELS - 1) * gap) / 2);
  board.forEach((row, r) => {
    const cell = UNIT * P ** (LEVELS - 1 - r);
    const space = SPACES[r];
    row.forEach((state, col) => {
      if (state !== null) pen.rect(x + col * cell + Math.floor(space / 2), top + r * (tall + gap), cell - space, tall, state === 0 ? ink.blue : ink.orange);
    });
  });
}
