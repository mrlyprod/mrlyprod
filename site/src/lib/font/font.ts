import GEN from "./font.gen.json" with { type: "json" };

/* TYPES */

export type Entry = { rows: string[]; path: number[][] };

export type Book = Record<string, Entry | undefined>;

export type Anim = { rows: number; cols: number; fps: number; frames: number[][] };

export type Letters = { rows: number; cols: number; grid: number[][] };

type Pair = [number, number];

type Block = { char: string; rows: string[]; col: number; offset: number };

type Layout = { width: number; height: number; blocks: Block[] };

/* BOOK */

const FONT: Book = GEN;
const FPS = 25;
const BLANK = ["000", "000", "000", "000", "000"];

/* GLYPHS */

function trim(rows: string[]): string[] {
  let from = Infinity;
  let to = 0;
  for (const row of rows) {
    for (let c = 0; c < row.length; c++) {
      if (row[c] !== "1") continue;
      from = Math.min(from, c);
      to = Math.max(to, c + 1);
    }
  }
  return from === Infinity ? BLANK : rows.map((row) => row.slice(from, to));
}

export const glyph = (char: string) => trim(FONT[char]?.rows ?? BLANK);

function layout(text: string): Layout {
  const blocks: Block[] = [];
  let col = 0;
  let height = 5;
  for (const char of text) {
    const rows = glyph(char);
    height = Math.max(height, rows.length);
    blocks.push({ char, rows, col, offset: 0 });
    col += rows[0].length + 1;
  }
  for (const block of blocks) block.offset = (height - block.rows.length) >> 1;
  return { width: blocks.length ? col - 1 : 0, height, blocks };
}

/* RASTER */

export function letters(text: string): Letters {
  const { width, height, blocks } = layout(text);
  const grid = Array.from({ length: width ? height : 0 }, () => Array<number>(width).fill(0));
  for (const block of blocks) {
    block.rows.forEach((row, r) => {
      for (let c = 0; c < row.length; c++) if (row[c] === "1") grid[block.offset + r][block.col + c] = 1;
    });
  }
  return { rows: grid.length, cols: width, grid };
}

/* STROKES */

const sequence = (char: string) => FONT[char]?.path ?? [];

/* ANIMATION */

const board = (laid: Layout, pad: number): Pair =>
  laid.blocks.length ? [laid.height + 2 * pad, laid.width + 2 * pad] : [0, 0];

export function animate(text: string, pad = 1): Anim {
  const laid = layout(text);
  const [rows, cols] = board(laid, pad);
  const frames: number[][] = [[]];
  const current: number[] = [];
  for (const block of laid.blocks) {
    for (const [r, c] of sequence(block.char)) {
      current.push((pad + block.offset + r) * cols + (pad + block.col + c));
      frames.push([...current].sort((a, b) => a - b));
    }
  }
  return { rows, cols, fps: FPS, frames };
}

function stamp(blocks: Block[], spots: Pair[], rows: number, cols: number): number[] {
  const active = new Set<number>();
  blocks.forEach((block, i) => {
    const [cx, cy] = spots[i];
    block.rows.forEach((row, r) => {
      for (let c = 0; c < row.length; c++) {
        if (row[c] !== "1") continue;
        const y = cy + r;
        const x = cx + c;
        if (y >= 0 && y < rows && x >= 0 && x < cols) active.add(y * cols + x);
      }
    });
  });
  return [...active].sort((a, b) => a - b);
}

function beat(frame: number, total: number, phases: number): Pair {
  const len = Math.floor(total / phases);
  for (let p = 1; p < phases; p++) if (frame < len * p) return [p, (frame - len * (p - 1)) / len];
  const done = len * (phases - 1);
  return [phases, (frame - done) / (total - done)];
}

const lerp = (start: number, end: number, p: number) => start + Math.trunc((end - start) * p);

function place(
  idx: number,
  n: number,
  phases: number,
  phase: number,
  progress: number,
  starts: Pair[],
  targets: Pair[],
): Pair {
  const slide = (a: Pair, b: Pair): Pair => [lerp(a[0], b[0], progress), lerp(a[1], b[1], progress)];
  if (phase < phases) {
    if (idx < phase) return slide(starts[phase - 1], starts[phase]);
    if (idx >= n - phase) return slide(starts[n - phase], starts[n - phase - 1]);
    return starts[idx];
  }
  const anchor = idx < phases ? starts[phases - 1] : idx >= n - phases ? starts[n - phases] : starts[idx];
  return slide(anchor, targets[idx]);
}

const same = (a: number[], b: number[]) => a.length === b.length && a.every((v, i) => v === b[i]);

export function merge(text: string, pad = 1): number[][] {
  const laid = layout(text);
  const [rows, cols] = board(laid, pad);
  const n = laid.blocks.length;
  const phases = n >> 1;
  const starts: Pair[] = laid.blocks.map((b) => [pad + b.col, pad + b.offset]);
  const targets: Pair[] = laid.blocks.map((b) => [Math.floor((cols - b.rows[0].length) / 2), pad + b.offset]);
  if (!phases) return [stamp(laid.blocks, starts, rows, cols)];
  const total = cols >> 1;
  const frames: number[][] = [];
  for (let i = 0; i <= total; i++) {
    const [phase, progress] = beat(i, total, phases);
    const spots = laid.blocks.map((_, idx) => place(idx, n, phases, phase, progress, starts, targets));
    const frame = stamp(laid.blocks, spots, rows, cols);
    if (!frames.length || !same(frames[frames.length - 1], frame)) frames.push(frame);
  }
  return frames;
}

export function fold(text: string, into = "X", pad = 0): Anim {
  const laid = layout(text);
  const [rows, cols] = board(laid, pad);
  const end = layout(into);
  const x = Math.floor((cols - end.width) / 2);
  const y = Math.floor((rows - end.height) / 2);
  const last = stamp(
    end.blocks,
    end.blocks.map((b) => [x + b.col, y + b.offset]),
    rows,
    cols,
  );
  return { rows, cols, fps: FPS, frames: [...merge(text, pad), last] };
}
