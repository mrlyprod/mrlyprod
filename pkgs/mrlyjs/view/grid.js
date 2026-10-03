// GRID

const round = (v) => (v < 0 ? -Math.round(-v) : Math.round(v));

export class Grid {
  constructor(frame, cols, rows, gap) {
    this.frame = frame;
    this.cols = Math.max(cols, 1);
    this.rows = Math.max(rows, 1);
    this.gap = gap;
  }

  cell(col, row) {
    const { x, y } = this.frame;
    const w = this.frame.w / this.cols;
    const h = this.frame.h / this.rows;
    if (this.gap <= 0) {
      const x0 = round(x + col * w);
      const x1 = round(x + (col + 1) * w);
      const y0 = round(y + row * h);
      const y1 = round(y + (row + 1) * h);
      return [x0, y0, x1 - x0, y1 - y0];
    }
    const pad = (this.gap * Math.min(w, h)) / 2;
    return [x + col * w + pad, y + row * h + pad, w - 2 * pad, h - 2 * pad];
  }

  fill(pen, col, row, color) {
    const [x, y, w, h] = this.cell(col, row);
    pen.rect(x, y, w, h, color);
  }

  paint(pen, cells, ink) {
    const [height, width] = cells.shape;
    for (let row = 0; row < Math.min(this.rows, height); row++) {
      for (let col = 0; col < Math.min(this.cols, width); col++) {
        const color = ink(cells.types[row * width + col] & 255);
        if (color) this.fill(pen, col, row, color);
      }
    }
  }

  carpet(pen, mask, color) {
    for (let row = 0; row < Math.min(mask.length, this.rows); row++) {
      for (let col = 0; col < Math.min(mask[row].length, this.cols); col++) {
        if (mask[row][col]) this.fill(pen, col, row, color);
      }
    }
  }
}

// MASKS

export const LOGO = Object.freeze(["11111", "10101", "11111", "10101", "11111"]);

export function mask(rows, level) {
  const seed = rows.map((row) => [...row].map((c) => c === "1"));
  const width = seed.length ? seed[0].length : 0;
  let out = seed;
  for (let step = 1; step < Math.max(level, 1); step++) {
    const next = [];
    for (const row of out) {
      for (const inner of seed) next.push(row.flatMap((on) => (on ? inner : new Array(width).fill(false))));
    }
    out = next;
  }
  return out;
}

export function carpet(pen, frame, mask, gap, color) {
  const rows = mask.length;
  const cols = rows ? mask[0].length : 0;
  if (rows === 0 || cols === 0) return;
  new Grid(frame, cols, rows, gap).carpet(pen, mask, color);
}
