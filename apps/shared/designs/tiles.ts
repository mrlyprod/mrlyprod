/* TYPES */

export type Tile = { size: number; cells: Uint8Array };

export type Rule = (r: number, c: number) => boolean;

export const DESIGNS = ["carpet", "net", "htree", "vtree", "void"];

export const NUMBERS = [3, 5, 7, 9];

export const LEVELS = [1, 2];

/* RULES */

const CARPET: Rule = (r, c) => (r & 1) + (c & 1) <= 1;

const RULES: Partial<Record<string, Rule>> = {
  carpet: CARPET,
  net: (r, c) => (r & 1) + (c & 1) >= 1,
  htree: (r) => (r & 1) === 0,
  vtree: (_r, c) => (c & 1) === 0,
  void: (r, c) => (r & 1) === (c & 1),
};

export function seed(design: string, n: number): Tile {
  const rule = RULES[design] ?? CARPET;
  const cells = new Uint8Array(n * n);
  for (let r = 0; r < n; r++) for (let c = 0; c < n; c++) cells[r * n + c] = rule(r, c) ? 1 : 0;
  return { size: n, cells };
}

/* GROWTH */

export function kron(base: Tile, level = 2): Tile {
  let out = base;
  const p = base.size;
  for (let step = 1; step < level; step++) {
    const size = out.size * p;
    const cells = new Uint8Array(size * size);
    for (let r = 0; r < size; r++) {
      const up = ((r / p) | 0) * out.size;
      const lo = (r % p) * p;
      for (let c = 0; c < size; c++) cells[r * size + c] = out.cells[up + ((c / p) | 0)] & base.cells[lo + (c % p)];
    }
    out = { size, cells };
  }
  return out;
}

export function tile(design: string, n = NUMBERS[0], level = 1): Tile {
  return kron(seed(design, n), level);
}

/* INK */

export function ink(ctx: CanvasRenderingContext2D, mark: Tile, x: number, y: number, side: number) {
  const n = mark.size;
  ctx.beginPath();
  for (let r = 0; r < n; r++) {
    const top = Math.round(y + (r * side) / n);
    const bottom = Math.round(y + ((r + 1) * side) / n);
    for (let c = 0; c < n; c++) {
      if (!mark.cells[r * n + c]) continue;
      const left = Math.round(x + (c * side) / n);
      ctx.rect(left, top, Math.round(x + ((c + 1) * side) / n) - left, bottom - top);
    }
  }
  ctx.fill();
}
