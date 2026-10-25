/* TYPES */

export type Cell = { shape: readonly number[]; types: ArrayLike<number> };

export type Run = [number, number, number];

export type Mark = [number, number];

export type Stroke = {
  points: ArrayLike<number>;
  color?: string;
  width?: number;
  dash?: readonly number[];
  closed?: boolean;
};

export type Lines = { width: number; height: number; strokes?: readonly Stroke[] };

export type Rects = { fill?: string; ground?: string; unit?: number };

export const NS = "http://www.w3.org/2000/svg";

export const num = (x: number, places = 2) => String(Math.round(x * 10 ** places) / 10 ** places);

export function sheet(width: number, height: number, body: string, viewBox = `0 0 ${num(width)} ${num(height)}`) {
  return `<svg xmlns="${NS}" width="${num(width)}" height="${num(height)}" viewBox="${viewBox}">${body}</svg>`;
}

/* CELLS */

export function runs(cell: Cell): Run[] {
  const [rows, cols] = cell.shape;
  const out: Run[] = [];
  for (let row = 0; row < rows; row++) {
    let start = -1;
    for (let col = 0; col <= cols; col++) {
      const on = col < cols && cell.types[row * cols + col];
      if (on && start < 0) start = col;
      if (!on && start >= 0) {
        out.push([row, start, col - start]);
        start = -1;
      }
    }
  }
  return out;
}

export function marks(cells: Iterable<number>, cols: number): Mark[] {
  return cols > 0 ? Array.from(cells, (i): Mark => [i % cols, Math.floor(i / cols)]) : [];
}

export function grid(rows: number, cols: number, cells: Iterable<number>): Cell {
  const types = new Uint8Array(rows * cols);
  for (const i of cells) types[i] = 1;
  return { shape: [rows, cols], types };
}

export function rects(cell: Cell, { fill = "#000", ground = "", unit = 10 }: Rects = {}) {
  const [rows, cols] = cell.shape;
  const w = cols * unit;
  const h = rows * unit;
  const body = runs(cell).map(
    ([row, col, len]) =>
      `<rect x="${num(col * unit)}" y="${num(row * unit)}" width="${num(len * unit)}" height="${num(unit)}"/>`,
  );
  const back = ground ? `<rect width="${num(w)}" height="${num(h)}" fill="${ground}"/>` : "";
  return sheet(w, h, `${back}<g fill="${fill}" shape-rendering="crispEdges">${body.join("")}</g>`);
}

/* STROKES */

export function path(points: ArrayLike<number>, closed = false) {
  const out: string[] = [];
  for (let i = 0; i + 1 < points.length; i += 2) out.push(`${i ? "L" : "M"}${num(points[i])} ${num(points[i + 1])}`);
  if (closed && out.length) out.push("Z");
  return out.join("");
}

export function lines({ width, height, strokes = [] }: Lines) {
  const body = strokes.map(({ points, color, width: thick = 1, dash, closed }) => {
    const d = path(points, closed);
    if (!d) return "";
    const dashed = dash ? ` stroke-dasharray="${dash.map(num).join(" ")}"` : "";
    return `<path d="${d}" fill="none" stroke="${color}" stroke-width="${num(thick)}" stroke-linecap="round" stroke-linejoin="round"${dashed}/>`;
  });
  return sheet(width, height, body.join(""));
}
