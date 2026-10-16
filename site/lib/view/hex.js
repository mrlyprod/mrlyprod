// GEOMETRY

const RATIO = 0.8660254037844386;

const north = (x, y) => [[x, 2 * y + 2], [x + 1, 2 * y], [x + 2, 2 * y + 2]];
const south = (x, y) => [[x, 2 * y], [x + 1, 2 * y + 2], [x + 2, 2 * y]];
const east = (x, y) => [[2 * x, y], [2 * x, y + 2], [2 * x + 2, y + 1]];
const west = (x, y) => [[2 * x + 2, y], [2 * x + 2, y + 2], [2 * x, y + 1]];

function shrink(pts, gap) {
  const cx = (pts[0][0] + pts[1][0] + pts[2][0]) / 3;
  const cy = (pts[0][1] + pts[1][1] + pts[2][1]) / 3;
  const side = (a, b) => Math.sqrt((b[0] - a[0]) * (b[0] - a[0]) + (b[1] - a[1]) * (b[1] - a[1]));
  const perimeter = side(pts[0], pts[1]) + side(pts[1], pts[2]) + side(pts[2], pts[0]);
  const area = Math.abs((pts[1][0] - pts[0][0]) * (pts[2][1] - pts[0][1]) - (pts[2][0] - pts[0][0]) * (pts[1][1] - pts[0][1])) / 2;
  const inradius = (2 * area) / perimeter;
  const k = inradius > 0 ? Math.max((inradius - gap) / inradius, 0) : 0;
  return pts.map((p) => [cx + (p[0] - cx) * k, cy + (p[1] - cy) * k]);
}

function fit(mesh, frame) {
  let [lx, ly, hx, hy] = [Number.MAX_VALUE, Number.MAX_VALUE, -Number.MAX_VALUE, -Number.MAX_VALUE];
  for (const tri of mesh) {
    for (const p of tri) {
      if (p[0] < lx) lx = p[0];
      if (p[1] < ly) ly = p[1];
      if (p[0] > hx) hx = p[0];
      if (p[1] > hy) hy = p[1];
    }
  }
  const sx = Math.max(hx - lx, 1e-9);
  const sy = Math.max(hy - ly, 1e-9);
  const scale = Math.min(frame.w / sx, frame.h / sy);
  const ox = frame.x + (frame.w - sx * scale) / 2;
  const oy = frame.y + (frame.h - sy * scale) / 2;
  return (p) => [ox + (p[0] - lx) * scale, oy + (p[1] - ly) * scale];
}

// DRAWING

export function draw(pen, frame, cell, gap, ink) {
  const [height, width] = cell.cell.shape;
  if (width === height) throw new Error("Cell must be a hexagon.");
  const across = width > height;
  const types = cell.cell.types;
  const mesh = [];
  const paint = [];
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const color = ink(types[y * width + x] & 255);
      if (!color) continue;
      const flip = (x + y + cell.start) % 2;
      const points = across ? (flip === 0 ? north(x, y) : south(x, y)) : flip === 0 ? east(x, y) : west(x, y);
      mesh.push(points.map((p) => (across ? [p[0], p[1] * RATIO] : [p[0] * RATIO, p[1]])));
      paint.push(color);
    }
  }
  if (mesh.length === 0) return;
  const place = fit(mesh, frame);
  mesh.forEach((tri, i) => {
    const small = shrink(tri.map(place), gap);
    pen.triangle(small[0], small[1], small[2], paint[i]);
  });
}

export function count(n) {
  return 6 * n * n;
}

export function row_len(n, row) {
  const reach = row < n ? row : 2 * n - 1 - row;
  return 2 * (n + reach) + 1;
}

export function at(slice, side, row, col) {
  const width = slice.cell.shape[1];
  return slice.cell.types[row * width + col + Math.floor((width - row_len(side, row)) / 2)];
}

export function hexagon(pen, frame, n, gap, ink) {
  if (n === 0) return;
  const side = Math.min(frame.w / (2 * n), frame.h / (n * 2 * RATIO));
  const rise = side * RATIO;
  const [cx, cy] = frame.center();
  const left = cx - side * n;
  const top = cy - rise * n;
  for (let row = 0; row < 2 * n; row++) {
    const reach = row < n ? row : 2 * n - 1 - row;
    const up = row < n;
    const long = n + reach + 1;
    const short = n + reach;
    const [topLen, botLen] = up ? [short, long] : [long, short];
    const tx = left + ((2 * n - topLen) * side) / 2;
    const bx = left + ((2 * n - botLen) * side) / 2;
    const y0 = top + row * rise;
    const y1 = top + (row + 1) * rise;
    for (let col = 0; col < row_len(n, row); col++) {
      const pointing = (col % 2 === 0) === up;
      const j = Math.floor(col / 2);
      const points = pointing
        ? [[bx + j * side, y1], [bx + (j + 1) * side, y1], [bx + (j + 0.5) * side, y0]]
        : [[tx + j * side, y0], [tx + (j + 1) * side, y0], [tx + (j + 0.5) * side, y1]];
      const color = ink(row, col, pointing ? 1 : 0);
      if (color) {
        const small = shrink(points, gap);
        pen.triangle(small[0], small[1], small[2], color);
      }
    }
  }
}
