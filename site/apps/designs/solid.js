export const PX = 0;
export const NX = 1;
export const PY = 2;
export const NY = 3;
export const PZ = 4;
export const NZ = 5;

const CORNERS = [
  [[1, 0, 0], [1, 1, 0], [1, 1, 1], [1, 0, 1]],
  [[0, 0, 0], [0, 0, 1], [0, 1, 1], [0, 1, 0]],
  [[0, 1, 0], [0, 1, 1], [1, 1, 1], [1, 1, 0]],
  [[0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1]],
  [[0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]],
  [[0, 0, 0], [0, 1, 0], [1, 1, 0], [1, 0, 0]],
];

const SWELL = 0.35;
const LIGHT = 0.62;
const SPREAD = 0.2;

export const PHI = Math.atan(Math.SQRT1_2);

/* CELLS */

export function solid(quads, n) {
  const seen = new Map();
  for (const { normal, verts } of quads) {
    const axis = normal.x ? 0 : normal.y ? 1 : 2;
    const sign = (axis === 0 ? normal.x : axis === 1 ? normal.y : normal.z) > 0;
    const sum = [0, 0, 0];
    for (const v of verts) {
      sum[0] += v.x;
      sum[1] += v.y;
      sum[2] += v.z;
    }
    const at = sum.map((one) => ((one / 4 + 1) / 2) * n);
    const cell = at.map((value, i) => (i === axis ? Math.round(value) - (sign ? 1 : 0) : Math.floor(value)));
    const key = (cell[0] * n + cell[1]) * n + cell[2];
    seen.set(key, (seen.get(key) ?? 0) | (1 << (axis * 2 + (sign ? 0 : 1))));
  }
  const cells = new Int32Array(seen.size);
  const masks = new Uint8Array(seen.size);
  let i = 0;
  for (const [key, mask] of seen) {
    cells[i] = key;
    masks[i] = mask;
    i++;
  }
  return { n, cells, masks };
}

export function exposed(shape) {
  let count = 0;
  for (const mask of shape.masks) for (let bit = 0; bit < 6; bit++) if (mask & (1 << bit)) count++;
  return count;
}

/* CAMERA */

export function camera(theta, phi, scale) {
  const c = Math.cos(theta);
  const s = Math.sin(theta);
  const cp = Math.cos(phi);
  const sp = Math.sin(phi);
  const ex = [c * scale, -s * sp * scale];
  const ey = [-s * scale, -c * sp * scale];
  const ez = [0, -cp * scale];
  const visible = (1 << PZ) | (s < 0 ? 1 << PX : 0) | (s > 0 ? 1 << NX : 0) | (c < 0 ? 1 << PY : 0) | (c > 0 ? 1 << NY : 0);
  const across = [c, -c, -s, s, 0, 0];
  const tone = across.map((nx, face) => (face === PZ ? 1 : face === NZ ? 0 : LIGHT + SPREAD * nx));
  const shapes = CORNERS.map((corners) => {
    const pts = corners.map(([x, y, z]) => [x * ex[0] + y * ey[0] + z * ez[0], x * ex[1] + y * ey[1] + z * ez[1]]);
    const mid = pts.reduce(([mx, my], [x, y]) => [mx + x / 4, my + y / 4], [0, 0]);
    return pts.map(([x, y]) => {
      const dx = x - mid[0];
      const dy = y - mid[1];
      const len = Math.hypot(dx, dy) || 1;
      return [x + (dx / len) * SWELL, y + (dy / len) * SWELL];
    });
  });
  const depth = (x, y, z) => (x * s + y * c) * cp - z * sp;
  return { ex, ey, ez, visible, tone, shapes, depth, at: (x, y, z) => [x * ex[0] + y * ey[0] + z * ez[0], x * ex[1] + y * ey[1] + z * ez[1]] };
}

export function order(shape, cam) {
  const { n, cells, masks } = shape;
  const half = n / 2;
  const keys = new Float32Array(cells.length);
  const picked = [];
  for (let i = 0; i < cells.length; i++) {
    if (!(masks[i] & cam.visible)) continue;
    const key = cells[i];
    const x = Math.floor(key / (n * n));
    const y = Math.floor(key / n) % n;
    const z = key % n;
    keys[i] = cam.depth(x - half, y - half, z - half);
    picked.push(i);
  }
  picked.sort((a, b) => keys[b] - keys[a]);
  return picked;
}

export function walk(shape, cam, cx, cy, emit) {
  const { n, cells, masks } = shape;
  const half = n / 2;
  const { ex, ey, ez, visible, shapes } = cam;
  let count = 0;
  for (const i of order(shape, cam)) {
    const key = cells[i];
    const x = Math.floor(key / (n * n)) - half;
    const y = (Math.floor(key / n) % n) - half;
    const z = (key % n) - half;
    const ox = cx + x * ex[0] + y * ey[0] + z * ez[0];
    const oy = cy + x * ex[1] + y * ey[1] + z * ez[1];
    const mask = masks[i] & visible;
    for (let face = 0; face < 6; face++) {
      if (!(mask & (1 << face))) continue;
      const [a, b, c, d] = shapes[face];
      emit(face, ox + a[0], oy + a[1], ox + b[0], oy + b[1], ox + c[0], oy + c[1], ox + d[0], oy + d[1]);
      count++;
    }
  }
  return count;
}

export function extent(n, phi) {
  return [n * Math.SQRT2, n * (Math.SQRT2 * Math.sin(phi) + Math.cos(phi))];
}
